import {useState} from 'react';
import * as Form from '@radix-ui/react-form';
import '../../../shared/form.css';

export default function FormPage() {
    const [data, setData] = useState('{}');
    const [serverErrors, setServerErrors] = useState({name: false});

    const onSubmit = (event) => {
        event.preventDefault();

        const formData = new FormData(event.currentTarget);
        const name = formData.get('name') || '';
        const email = formData.get('email') || '';

        // Simulate async server validation: name must not be "taken"
        // Uses setTimeout because onClearServerErrors fires synchronously after onSubmit,
        // so synchronous setServerErrors would be immediately cleared by the batch.
        if (name === 'taken') {
            setTimeout(() => setServerErrors({name: true}), 0);
            return;
        }

        setData(JSON.stringify({name, email}));
    };

    return (
        <>
            <Form.Root
                className="form-root"
                onClearServerErrors={() => setServerErrors({name: false})}
                onSubmit={onSubmit}
            >
                <Form.Field name="name" className="form-field" serverInvalid={serverErrors.name}>
                    <Form.Label className="form-label">Name</Form.Label>
                    <Form.Control className="form-control" type="text" required />
                    <Form.Message className="form-message" match="valueMissing">
                        Name is required
                    </Form.Message>
                    {serverErrors.name ? (
                        <Form.Message className="form-message">Name is already taken</Form.Message>
                    ) : null}
                </Form.Field>

                <Form.Field name="email" className="form-field">
                    <Form.Label className="form-label">Email</Form.Label>
                    <Form.Control className="form-control" type="email" required />
                    <Form.Message className="form-message" match="valueMissing">
                        Email is required
                    </Form.Message>
                    <Form.Message className="form-message" match="typeMismatch">
                        Please enter a valid email
                    </Form.Message>
                </Form.Field>

                <Form.Submit className="form-submit">Submit</Form.Submit>
                <button type="reset" onClick={() => setData('{}')}>
                    reset
                </button>
            </Form.Root>

            <pre data-testid="form-result">Data: {data}</pre>

            <button
                data-testid="outside-button"
                onClick={() => {
                    /* outside interaction target */
                }}
            >
                outside
            </button>

            <hr />

            <h3>ValidityState</h3>
            <Form.Root className="form-root" data-testid="validity-form" onSubmit={(e) => e.preventDefault()}>
                <Form.Field name="vs-name" className="form-field">
                    <Form.Label className="form-label">VS Name</Form.Label>
                    <Form.Control className="form-control" type="text" required data-testid="vs-name-input" />
                    <Form.ValidityState>
                        {(validity) => (
                            <span data-testid="vs-name-validity">
                                {validity ? JSON.stringify({valueMissing: validity.valueMissing, valid: validity.valid}) : 'undefined'}
                            </span>
                        )}
                    </Form.ValidityState>
                </Form.Field>

                <Form.Field name="vs-email" className="form-field">
                    <Form.Label className="form-label">VS Email</Form.Label>
                    <Form.Control className="form-control" type="email" data-testid="vs-email-input" />
                    <Form.ValidityState>
                        {(validity) => (
                            <span data-testid="vs-email-validity">
                                {validity ? JSON.stringify({typeMismatch: validity.typeMismatch, valid: validity.valid}) : 'undefined'}
                            </span>
                        )}
                    </Form.ValidityState>
                </Form.Field>

                <Form.Submit className="form-submit" data-testid="vs-submit">Check Validity</Form.Submit>
            </Form.Root>
        </>
    );
}
