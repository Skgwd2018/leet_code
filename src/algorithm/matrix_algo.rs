// 矩阵遍历涉及使用不同的技(如DFS、BFS等)遍历矩阵中的元素。在处理涉及横向、纵向或对角线遍历二维网格或矩阵的问题时，可使用该模式。
// 示例问题:对二维网格进行颜色填充。将与起始单元格相连通的所有单元格都更改为新颜色。
// 示例：输入:image = [[1,1,1], [1,1,0], [1,0,1]]，sr = 1，sc = 1，newColor = 2 输出：[[2,2,2], [2,2,0], [2,0,1]]
// 解释：使用DFS或BFS从给定单元格开始遍历矩阵。将连通单元格的颜色更改为新颜色。
pub fn matrix_ex(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) {
    let original_color = image[sr][sc];
    if original_color != new_color {
        dfs(image, sr, sc, original_color, new_color);
    }
}

fn dfs(image: &mut Vec<Vec<i32>>, r: usize, c: usize, original_color: i32, new_color: i32) {
    let (rows, cols) = (image.len(), image[0].len());

    // 检查边界条件和颜色匹配
    if r >= rows || c >= cols || image[r][c] != original_color { return; }

    // 将当前单元格的颜色更改为新颜色
    image[r][c] = new_color;

    // 递归地对上、下、左、右四个方向的相邻单元格进行DFS
    if r > 0 { dfs(image, r - 1, c, original_color, new_color); } // 上
    dfs(image, r + 1, c, original_color, new_color); // 下
    if c > 0 { dfs(image, r, c - 1, original_color, new_color); } // 左
    dfs(image, r, c + 1, original_color, new_color); // 右
}