use std::collections::HashMap;
use image::DynamicImage;

// 指定された要件に基づいて、A～Zのアルファベットが横に並んだ画像を一文字ずつDynamicImageに切り分けて、それをHashMapに保存して、文章を受け取る関数を作成する手順を以下に示します。
// まず、アルファベット画像の1文字の幅を知る必要があります。仮定として、すべての文字が同じ幅であるとします。全体の幅を26で割ることで、一文字の幅を計算できます。
// 各アルファベットをDynamicImageに切り分けます。
// 切り分けた各画像をHashMapに保存します。
// 文章を受け取り、文字ごとに対応するDynamicImageを返す関数を作成します。


pub fn split_alphabet_image(alphabet_image: &DynamicImage) -> HashMap<char, DynamicImage> {
    let mut alphabet_map = HashMap::new();

    let width = alphabet_image.width();
    let height = alphabet_image.height();
    let width_per_char = width / 26;

    for (i, character) in ('A'..'Z').enumerate() {
        let x = i as u32 * width_per_char;
        let character_image = alphabet_image.crop_imm(x, 0, width_per_char, height);
        alphabet_map.insert(character, character_image);
    }

    alphabet_map
}

pub fn get_sentence_images(sentence: &str, map: &HashMap<char, DynamicImage>) -> Vec<DynamicImage> {
    sentence.chars().filter_map(|ch| map.get(&ch)).cloned().collect()
}



// fn main() {
//     let alphabet_image = DynamicImage::new_rgba8(2600, 100);  // 仮のアルファベット画像
//     let map = split_alphabet_image(alphabet_image);

//     let sentence = "hello";
//     let sentence_images = get_sentence_images(sentence, &map);
    
//     // ここでsentence_imagesを使用して描画などを行う
// }