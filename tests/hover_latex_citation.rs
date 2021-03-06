use texlab_protocol::*;
use texlab_test::hover::*;

const SCENARIO: &str = "latex/citation";

#[tokio::test]
async fn valid() {
    let contents = run(SCENARIO, "foo.tex", 2, 7).await.unwrap();
    assert_eq!(
        contents,
        HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Bar, F. (2019). *Baz Qux*.".into()
        })
    );
}

#[tokio::test]
async fn invalid() {
    let contents = run(SCENARIO, "foo.bib", 3, 7).await;
    assert_eq!(contents, None);
}
