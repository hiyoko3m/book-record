use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionTitleProps {
    pub title: String,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &SectionTitleProps) -> Html {
    html! {
        <h3 class="sec-title">{ props.title.clone() } </h3>
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub message: String,
    pub on_click: Callback<MouseEvent>,
    pub additional_class: Option<Vec<String>>,
}

#[function_component(ButtonSecondary)]
pub fn button_secondary(
    ButtonProps {
        message,
        on_click,
        additional_class,
    }: &ButtonProps,
) -> Html {
    let class = if let Some(additional_class) = additional_class {
        format!("btn btn-secondary {}", additional_class.join(" "))
    } else {
        "btn btn-secondary".to_string()
    };

    html! {
        <button class={class} onclick={on_click.clone()}>{ message }</button>
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalButtonProps {
    pub message: String,
    pub on_click: Callback<MouseEvent>,
    pub additional_class: Option<Vec<String>>,
    pub modal_target: String,
}

#[function_component(ModalButtonSecondary)]
pub fn modal_button_secondary(
    ModalButtonProps {
        message,
        on_click,
        additional_class,
        modal_target,
    }: &ModalButtonProps,
) -> Html {
    let class = if let Some(additional_class) = additional_class {
        format!("btn btn-secondary {}", additional_class.join(" "))
    } else {
        "btn btn-secondary".to_string()
    };

    html! {
        <button class={class} onclick={on_click.clone()} data-bs-toggle="modal" data-bs-target={modal_target.clone()}>{ message }</button>
    }
}
