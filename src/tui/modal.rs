#[derive(Debug)]
pub(super) enum Modal {
    SearchModal {
        r#type: SearchModalType,
        query: String,
    },
}

#[derive(Debug)]
pub(super) enum SearchModalType {
    Function,
}
