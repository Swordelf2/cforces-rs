# TODO
* Add `fn solve()` to cforces template
* Seperate toolchain for atcoder with their version of Rust
* Fix AtCoder 
* Solve interactive problem on AtCoder
* Solve an interactive problem on cforces (check input/output)
* Solve AtCoder E
* LeetCode
* Check "drop-in replacements for lower_bound(),...) from EbTech's work

# Future work on input/output
* Find better way for /* Library */ and /* Solution */ and stuff (for these comments)
* Use Result and proper errors and error handling
* Add easy flushing

# DFS postorder
```rust
let dfs_postorder = {
            let mut visited = vec![false; n];
            let mut res = Vec::with_capacity(n);
            visited[0] = true;
            fn dfs_postorder(
                g: &[Vec<usize>],
                visited: &mut Vec<bool>,
                res: &mut Vec<usize>,
                u: usize,
            ) {
                for &v in &g[u] {
                    if !visited[v] {
                        visited[v] = true;
                        dfs_postorder(&g, visited, res, v);
                    }
                }
                res.push(u);
            }
            dfs_postorder(&g, &mut visited, &mut res, root);
            res
        };

        dbg!(dfs_postorder.len());
        for (i, val) in dfs_postorder.iter().enumerate() {
            dbg!(val + 1);
        }
```