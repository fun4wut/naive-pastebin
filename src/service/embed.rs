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
    <link rel=\"stylesheet\" href=\"http://{}/static/css/prism.css\">
    <script src=\"http://{}/static/js/prism.js\"></script>
    <div class=\"panel\">
    <pre class=\"line-numbers\"><code class=\"lang-{}\">{}</code></pre>
    <div class=\"meta\">
    <a href=\"/raw/{}/{}\" style=\"float:right\">view raw</a>
    <a href=\"\">{}</a>
    hosted with ❤ by <a href=\"https://blog.fun4go.top\">番茄瓜皮</a>
    </div>
    </div>
    `)", *DOMAIN, *DOMAIN, item.value.lang(), item.value.escape(), key, item.value.title.clone(), item.value.title.clone());
    Ok(res)
}