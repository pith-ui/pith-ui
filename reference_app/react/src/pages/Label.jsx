import * as Label from '@radix-ui/react-label';

export default function LabelPage() {
    return (
        <>
            <Label.Root data-testid="basic-label" htmlFor="basic-input">
                Basic Label
            </Label.Root>
            <input id="basic-input" data-testid="basic-input" type="text" />

            <br />
            <br />

            <Label.Root data-testid="label-with-button">
                Label with button{' '}
                <button data-testid="nested-button">Click me</button>
            </Label.Root>

            <br />
            <br />

            <Label.Root data-testid="label-with-input">
                Label with input{' '}
                <input data-testid="nested-input" type="text" />
            </Label.Root>
        </>
    );
}
