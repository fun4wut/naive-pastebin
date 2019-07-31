use rocket::State;
use crate::core::StoreLock;
use crate::utils::error::StoreError;
use crate::domain::key::key_to_nano;

/// 生成嵌入的JS代码
pub fn gen_embed(state: State<StoreLock>, key: String) -> Result<String, StoreError> {
    let mut store = state.write().unwrap();

    let nano = key_to_nano(&key)?;
    let item = store.access(nano)?;

    let res = format!("document.write(`
    <link href=\"http://apps.bdimg.com/libs/highlight.js/9.1.0/styles/default.min.css\" rel=\"stylesheet\">
    <script src=\"http://apps.bdimg.com/libs/highlight.js/9.1.0/highlight.min.js\"></script>
    <pre><code class=\"{}\">{}</code></pre>
    <div class=\"meta\">
    <a href=\"\" style=\"float:right\">view raw</a>
    <a href=\"\">{}</a>
    hosted with ❤ by <a href=\"https://github.com\">GitHub</a>
    </div>
    <script>hljs.initHighlightingOnLoad();</script>
    `)", item.value.lang(), item.value.content.clone(), item.value.title.clone());
    Ok(res)
}