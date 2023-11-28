// cargo login https://crates.io/settings/tokens 中获取的 token
// cargo login 会将你的 token 存储在本地 ~/.cargo/credentials 中
// cargo publish 发布,发布 workspaces 时必须手动进入每个包目录 运行 cargo publish 将每个包进行发布
// cargo yank --vers 1.0.1 撤回某个版本
// cargo yank --vers 1.0.1 --undo 取消撤回某个版本
pub fn use_crate_login() {}
