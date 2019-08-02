use rocket::State;
use crate::core::StoreLock;
use crate::utils::error::StoreError;
use crate::domain::key::key_to_nano;
use crate::utils::env::*;
/// 生成嵌入的JS代码
pub fn gen_embed(state: State<StoreLock>, key: String) -> Result<String, StoreError> {
    let mut store = state.write().unwrap();

    let nano = key_to_nano(&key)?;
    let item = store.access(nano)?;

    // 字符串手撕，暴力而丑陋
    let res = format!("document.write(`
    <link rel=\"stylesheet\" href=\"http://{}/static/css/embed.css\">
    <script src=\"http://{}/static/js/highlight.pack.js\"></script>
    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlightjs-line-numbers.js/2.7.0/highlightjs-line-numbers.min.js\"></script>
    <div class=\"panel\">
    <pre><code>{}</code></pre>
    <div class=\"meta\">
    <a href=\"http://{}/raw/{}/{}\" style=\"float:right\">view raw</a>
    <a href=\"http://{}/show/{}\">{}</a>
    hosted with ❤ by <a href=\"https://blog.fun4go.top\">番茄瓜皮</a>
    </div>
    </div>
    <script>
      hljs.initHighlightingOnLoad();
      hljs.initLineNumbersOnLoad();
    </script>
    `)",
                      *DOMAIN,
                      *DOMAIN,
                      item.value.escape(),
                      *DOMAIN, &key, &item.value.title,
                      *DOMAIN, &key, &item.value.title);
    Ok(res)
}