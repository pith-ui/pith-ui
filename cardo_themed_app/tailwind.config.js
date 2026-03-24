/** @type {import('tailwindcss').Config} */

// Generate a scale object mapping 1-12 to CSS custom properties.
function scale(prefix) {
    const entries = {};
    for (let i = 1; i <= 12; i++) {
        entries[i] = `var(--${prefix}-${i})`;
    }
    return entries;
}

// Generate alpha scale (a1-a12) for a prefix.
function alphaScale(prefix) {
    const entries = {};
    for (let i = 1; i <= 12; i++) {
        entries[`a${i}`] = `var(--${prefix}-a${i})`;
    }
    return entries;
}

module.exports = {
    content: ['*.html', './src/**/*.rs'],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // --- Scales (mapped from Radix Colors via tokens.css) ---
                accent: { ...scale('accent'), ...alphaScale('accent') },
                neutral: { ...scale('neutral'), ...alphaScale('neutral') },
                danger: { ...scale('danger') },
                success: { ...scale('success') },

                // --- Semantic one-offs ---
                page: 'var(--color-page)',
                'focus-ring': 'var(--color-focus-ring)',
            },
            borderRadius: {
                1: 'var(--radius-1)',
                2: 'var(--radius-2)',
                3: 'var(--radius-3)',
                4: 'var(--radius-4)',
            },
            boxShadow: {
                1: 'var(--shadow-1)',
                2: 'var(--shadow-2)',
                3: 'var(--shadow-3)',
            },
            transitionDuration: {
                fast: 'var(--duration-fast)',
                normal: 'var(--duration-normal)',
                slow: 'var(--duration-slow)',
            },
        },
    },
    plugins: [],
    corePlugins: {
        preflight: false,
    },
};
