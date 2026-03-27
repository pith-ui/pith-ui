use leptos::prelude::*;

use crate::theme::button::Button;
use crate::theme::form::*;

#[component]
pub fn FormPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Form"</h1>
                <p class="text-muted-foreground mb-6">
                    "Accessible form validation with native constraint API integration and custom error messages."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Validation Form"</h2>
                <p class="text-sm text-muted-foreground">
                    "Try submitting with empty fields or an invalid email to see validation messages."
                </p>
                <div class="w-[360px]">
                    <ThemedForm>
                        <ThemedFormField name="name">
                            <ThemedFormLabel>"Name"</ThemedFormLabel>
                            <ThemedFormControl
                                attr:class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs outline-none placeholder:text-muted-foreground md:text-sm dark:bg-input/30 focus-visible:focus-ring"
                                attr:r#type="text"
                                attr:required=""
                                attr:placeholder="Enter your name"
                            />
                            <ThemedFormMessage r#match=ValidityMatcher::ValueMissing>
                                "Please enter your name."
                            </ThemedFormMessage>
                        </ThemedFormField>

                        <ThemedFormField name="email">
                            <ThemedFormLabel>"Email"</ThemedFormLabel>
                            <ThemedFormControl
                                attr:class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs outline-none placeholder:text-muted-foreground md:text-sm dark:bg-input/30 focus-visible:focus-ring"
                                attr:r#type="email"
                                attr:required=""
                                attr:placeholder="Enter your email"
                            />
                            <ThemedFormMessage r#match=ValidityMatcher::ValueMissing>
                                "Please enter your email."
                            </ThemedFormMessage>
                            <ThemedFormMessage r#match=ValidityMatcher::TypeMismatch>
                                "Please enter a valid email address."
                            </ThemedFormMessage>
                        </ThemedFormField>

                        <ThemedFormSubmit>
                            <Button>"Submit"</Button>
                        </ThemedFormSubmit>
                    </ThemedForm>
                </div>
            </section>
        </div>
    }
}
