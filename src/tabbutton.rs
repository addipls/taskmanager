
#[function_component(TabButton)]
pub fn tab_button(props: &TabButtonProps) -> Html {
    html! {
        <button class="tabButton">{props.text}</button>
    }
}