#!/usr/bin/env node

/**
 * Story Markup Comparison Tool
 *
 * Captures rendered HTML from both running Leptos (Trunk) and React (Storybook)
 * dev servers, extracts the relevant markup, and outputs a formatted comparison.
 *
 * Prerequisites:
 *   cd scripts/story-compare && npm install && npm run setup
 *
 * Usage:
 *   node scripts/story-compare/index.mjs <story-path> [options]
 *
 * Examples:
 *   node scripts/story-compare/index.mjs scroll-area/basic
 *   node scripts/story-compare/index.mjs progress/styled
 *   node scripts/story-compare/index.mjs progress/chromatic --leptos-port 8080 --react-port 6006
 *
 * Options:
 *   --leptos-port <port>   Leptos dev server port (default: 8080)
 *   --react-port <port>    React storybook port (default: 6006)
 *   --leptos-only           Only capture Leptos markup
 *   --react-only            Only capture React markup
 *   --raw                   Output raw HTML without formatting
 *   --wait <ms>             Extra wait time after load (default: 500)
 */

import { chromium } from 'playwright';
import { readFileSync, readdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = resolve(__dirname, '../..');
const REACT_STORIES_DIR = resolve(
    REPO_ROOT,
    'reference/react-radix-primitives/apps/storybook/stories'
);

// --- Argument parsing ---

function parseArgs() {
    const args = process.argv.slice(2);
    const opts = {
        storyPath: null,
        leptosPort: 8080,
        reactPort: 6006,
        leptosOnly: false,
        reactOnly: false,
        raw: false,
        wait: 500,
    };

    for (let i = 0; i < args.length; i++) {
        const arg = args[i];
        if (arg === '--leptos-port') opts.leptosPort = parseInt(args[++i]);
        else if (arg === '--react-port') opts.reactPort = parseInt(args[++i]);
        else if (arg === '--leptos-only') opts.leptosOnly = true;
        else if (arg === '--react-only') opts.reactOnly = true;
        else if (arg === '--raw') opts.raw = true;
        else if (arg === '--wait') opts.wait = parseInt(args[++i]);
        else if (!arg.startsWith('-')) opts.storyPath = arg;
    }

    if (!opts.storyPath) {
        console.error(
            'Usage: node index.mjs <story-path> [options]\n' +
                'Example: node index.mjs scroll-area/basic\n' +
                'Example: node index.mjs progress/styled --leptos-port 8080 --react-port 6006'
        );
        process.exit(1);
    }

    return opts;
}

// --- React storybook ID mapping ---

/**
 * Builds a map from component kebab-name to { title, category, titleSlug }.
 * Scans the React story files for `export default { title: '...' }`.
 */
function buildReactStoryMap() {
    const map = {};
    let files;
    try {
        files = readdirSync(REACT_STORIES_DIR);
    } catch {
        console.error(`Warning: Cannot read React stories directory: ${REACT_STORIES_DIR}`);
        return map;
    }

    for (const file of files) {
        if (!file.endsWith('.stories.tsx')) continue;
        const componentName = file.replace('.stories.tsx', ''); // e.g. "scroll-area"
        try {
            const content = readFileSync(resolve(REACT_STORIES_DIR, file), 'utf-8');
            const match = content.match(/title:\s*['"]([^'"]+)['"]/);
            if (match) {
                const title = match[1]; // e.g. "Components/ScrollArea"
                const [category, name] = title.split('/');
                // Storybook ID: lowercase category + lowercase name (no separators)
                const titleSlug = `${category.toLowerCase()}-${name.toLowerCase()}`;
                map[componentName] = { title, category, titleSlug };
            }
        } catch {
            // skip unreadable files
        }
    }
    return map;
}

/**
 * Gets the React storybook iframe URL for a given story path.
 */
function getReactUrl(storyPath, reactPort, storyMap) {
    const [component, story] = storyPath.split('/');
    const mapping = storyMap[component];
    if (!mapping) {
        // Fallback: assume "components" category, strip hyphens from component name
        const slug = `components-${component.replace(/-/g, '')}`;
        return `http://localhost:${reactPort}/iframe.html?id=${slug}--${story}&viewMode=story`;
    }
    return `http://localhost:${reactPort}/iframe.html?id=${mapping.titleSlug}--${story}&viewMode=story`;
}

function getLeptosUrl(storyPath, leptosPort) {
    return `http://localhost:${leptosPort}/${storyPath}`;
}

// --- HTML capture ---

async function captureHtml(page, url, selector, waitMs) {
    try {
        await page.goto(url, { waitUntil: 'networkidle', timeout: 15000 });
    } catch {
        try {
            await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 10000 });
        } catch (e) {
            return { error: `Failed to load ${url}: ${e.message}` };
        }
    }

    // Wait for the selector to appear
    try {
        await page.waitForSelector(selector, { timeout: 10000 });
    } catch {
        return { error: `Selector "${selector}" not found at ${url}` };
    }

    // Extra wait for async rendering
    if (waitMs > 0) {
        await page.waitForTimeout(waitMs);
    }

    const html = await page.$eval(selector, (el) => el.innerHTML);
    return { html };
}

// --- HTML formatting ---

function formatHtml(html) {
    // Simple pretty-printer: indent nested tags
    let indent = 0;
    const lines = [];
    // Split on tag boundaries
    const tokens = html.replace(/>\s*</g, '>\n<').split('\n');

    for (const token of tokens) {
        const trimmed = token.trim();
        if (!trimmed) continue;

        // Closing tag
        if (trimmed.startsWith('</')) {
            indent = Math.max(0, indent - 1);
            lines.push('  '.repeat(indent) + trimmed);
        }
        // Self-closing tag
        else if (trimmed.endsWith('/>')) {
            lines.push('  '.repeat(indent) + trimmed);
        }
        // Opening tag
        else if (trimmed.startsWith('<')) {
            lines.push('  '.repeat(indent) + trimmed);
            // Only increase indent if not a void element and has a matching close
            if (!trimmed.match(/<(br|hr|img|input|link|meta|area|base|col|embed|source|track|wbr)\b/i)) {
                indent++;
            }
        }
        // Text content
        else {
            lines.push('  '.repeat(indent) + trimmed);
        }
    }

    return lines.join('\n');
}

// --- HTML structural analysis ---

/**
 * Parses HTML into a simplified tree for comparison.
 * Uses the browser's DOMParser via Playwright.
 */
async function analyzeHtml(page, html) {
    return page.evaluate((htmlStr) => {
        const parser = new DOMParser();
        const doc = parser.parseFromString(`<div>${htmlStr}</div>`, 'text/html');
        const root = doc.body.firstElementChild;

        function walk(el) {
            if (!el) return null;
            const node = {
                tag: el.tagName.toLowerCase(),
                attrs: {},
                children: [],
                text: '',
            };

            // Collect attributes (skip class, skip style for comparison)
            for (const attr of el.attributes) {
                if (attr.name === 'class') continue; // CSS modules make these incomparable
                node.attrs[attr.name] = attr.value;
            }

            // Collect children
            for (const child of el.childNodes) {
                if (child.nodeType === Node.ELEMENT_NODE) {
                    node.children.push(walk(child));
                } else if (child.nodeType === Node.TEXT_NODE) {
                    const text = child.textContent.trim();
                    if (text) node.text += text;
                }
            }

            return node;
        }

        const children = [];
        for (const child of root.children) {
            children.push(walk(child));
        }
        return children;
    }, html);
}

function compareTrees(reactNodes, leptosNodes, path = '') {
    const diffs = [];
    const maxLen = Math.max(reactNodes.length, leptosNodes.length);

    for (let i = 0; i < maxLen; i++) {
        const r = reactNodes[i];
        const l = leptosNodes[i];
        const currentPath = `${path}[${i}]`;

        if (!r && l) {
            diffs.push(`${currentPath}: Extra in Leptos: <${l.tag}>`);
            continue;
        }
        if (r && !l) {
            diffs.push(`${currentPath}: Missing in Leptos: <${r.tag}>`);
            continue;
        }

        if (r.tag !== l.tag) {
            diffs.push(`${currentPath}: Tag mismatch: React <${r.tag}> vs Leptos <${l.tag}>`);
            continue;
        }

        const tagPath = `${path}<${r.tag}>[${i}]`;

        // Compare attributes
        const allAttrs = new Set([...Object.keys(r.attrs), ...Object.keys(l.attrs)]);
        for (const attr of allAttrs) {
            if (attr === 'style') continue; // skip inline styles for structural comparison
            const rv = r.attrs[attr];
            const lv = l.attrs[attr];
            if (rv === undefined) {
                diffs.push(`${tagPath}: Extra attr in Leptos: ${attr}="${lv}"`);
            } else if (lv === undefined) {
                diffs.push(`${tagPath}: Missing attr in Leptos: ${attr}="${rv}"`);
            } else if (rv !== lv) {
                diffs.push(`${tagPath}: Attr "${attr}" differs: React="${rv}" vs Leptos="${lv}"`);
            }
        }

        // Compare text content
        if (r.text !== l.text && r.text && l.text) {
            const rt = r.text.substring(0, 50);
            const lt = l.text.substring(0, 50);
            if (rt !== lt) {
                diffs.push(`${tagPath}: Text differs: React="${rt}" vs Leptos="${lt}"`);
            }
        }

        // Recurse into children
        if (r.children.length || l.children.length) {
            diffs.push(...compareTrees(r.children, l.children, tagPath));
        }
    }

    return diffs;
}

// --- Main ---

async function main() {
    const opts = parseArgs();
    const storyMap = buildReactStoryMap();

    const leptosUrl = getLeptosUrl(opts.storyPath, opts.leptosPort);
    const reactUrl = getReactUrl(opts.storyPath, opts.reactPort, storyMap);

    console.error(`Leptos URL: ${leptosUrl}`);
    console.error(`React URL:  ${reactUrl}`);
    console.error('');

    const browser = await chromium.launch();
    const page = await browser.newPage();

    let leptosHtml = null;
    let reactHtml = null;

    // Capture Leptos
    if (!opts.reactOnly) {
        console.error('Capturing Leptos markup...');
        const result = await captureHtml(page, leptosUrl, 'main', opts.wait);
        if (result.error) {
            console.error(`  ERROR: ${result.error}`);
        } else {
            leptosHtml = result.html;
            console.error(`  Captured ${leptosHtml.length} chars`);
        }
    }

    // Capture React
    if (!opts.leptosOnly) {
        console.error('Capturing React markup...');
        const result = await captureHtml(page, reactUrl, '#storybook-root', opts.wait);
        if (result.error) {
            console.error(`  ERROR: ${result.error}`);
        } else {
            reactHtml = result.html;
            console.error(`  Captured ${reactHtml.length} chars`);
        }
    }

    // Output
    if (reactHtml !== null) {
        console.log('=== REACT MARKUP ===');
        console.log(opts.raw ? reactHtml : formatHtml(reactHtml));
        console.log('');
    }

    if (leptosHtml !== null) {
        console.log('=== LEPTOS MARKUP ===');
        console.log(opts.raw ? leptosHtml : formatHtml(leptosHtml));
        console.log('');
    }

    // Structural comparison
    if (reactHtml !== null && leptosHtml !== null) {
        console.log('=== STRUCTURAL COMPARISON ===');
        console.error('Analyzing DOM structure...');

        const [reactTree, leptosTree] = await Promise.all([
            analyzeHtml(page, reactHtml),
            analyzeHtml(page, leptosHtml),
        ]);

        const diffs = compareTrees(reactTree, leptosTree);
        if (diffs.length === 0) {
            console.log('No structural differences found (excluding class and style attributes).');
        } else {
            console.log(`Found ${diffs.length} difference(s):\n`);
            for (const diff of diffs) {
                console.log(`  ${diff}`);
            }
        }
    }

    await browser.close();
}

main().catch((e) => {
    console.error(e);
    process.exit(1);
});
