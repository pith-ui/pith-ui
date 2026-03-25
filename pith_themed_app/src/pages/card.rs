use leptos::prelude::*;

use crate::theme::button::*;
use crate::theme::card::*;
use crate::theme::input::*;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Card"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a card with header, content, and footer."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Notification Card"</h2>
                <div class="max-w-md">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Notifications"</CardTitle>
                            <CardDescription>"You have 3 unread messages."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-3">
                                <div class="flex items-start gap-3">
                                    <div class="size-2 mt-1.5 rounded-full bg-primary" />
                                    <div>
                                        <p class="text-sm font-medium text-foreground">"Your call has been confirmed."</p>
                                        <p class="text-sm text-muted-foreground">"1 hour ago"</p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="size-2 mt-1.5 rounded-full bg-primary" />
                                    <div>
                                        <p class="text-sm font-medium text-foreground">"You have a new message!"</p>
                                        <p class="text-sm text-muted-foreground">"2 hours ago"</p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="size-2 mt-1.5 rounded-full bg-primary" />
                                    <div>
                                        <p class="text-sm font-medium text-foreground">"Your subscription is expiring soon!"</p>
                                        <p class="text-sm text-muted-foreground">"5 hours ago"</p>
                                    </div>
                                </div>
                            </div>
                        </CardContent>
                        <CardFooter>
                            <Button>"Mark all as read"</Button>
                        </CardFooter>
                    </Card>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Account Settings Card"</h2>
                <div class="max-w-md">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Account Settings"</CardTitle>
                            <CardDescription>"Update your account information below."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"Name"</label>
                                    <ThemedInput placeholder="Your name" value="Pedro Duarte" />
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"Email"</label>
                                    <ThemedInput r#type="email" placeholder="your@email.com" />
                                </div>
                            </div>
                        </CardContent>
                        <CardFooter>
                            <div class="flex gap-2 ml-auto">
                                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                                <Button>"Save"</Button>
                            </div>
                        </CardFooter>
                    </Card>
                </div>
            </section>
        </div>
    }
}
