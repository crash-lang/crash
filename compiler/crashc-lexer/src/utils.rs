/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

/// Returns (row, column)
pub fn get_pos(target_index: u32, input: String) -> (usize, usize) {
    let mut total_index = 0;
    let mut row = 0;
    let mut col = 0;

    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        row = i + 1;

        for (j, _) in line.chars().enumerate() {
            col = j + 1;
            if total_index == target_index {
                break;
            }

            total_index += 1;
        }
    }

    (row, col)
}