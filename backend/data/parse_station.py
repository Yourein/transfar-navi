import csv
import json
import os

def csv_to_json(csv_file_path: str, output_root: str = "./station"):
    with open(csv_file_path, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        
        keys = reader.fieldnames
        # ヘッダーからキーを取得
        if not keys or len(keys) != 4:
            print("invalid dataframe. Expected number of columns is more or less.")
            return
        key1, key2, key3, key4 = keys[0], keys[1], keys[2], keys[3]
        for row in reader:
            # 1列目の値からディレクトリとファイル名を決定
            path_parts = row[key1].split('_')
            filename = path_parts[-1] + '.json'
            dir_path = os.path.join(output_root, *path_parts[:-1])  # ← ここ修正

            # ディレクトリを作成（存在しなければ）
            os.makedirs(dir_path, exist_ok=True)

            # 3列目をセミコロンで分割してリスト化
            list_data = row[key3].split(';') if row[key3] else []

            # JSONデータ作成
            json_data = {
                key1: row[key1],
                key2: row[key2],
                key3: list_data,
                key4: row[key4]
            }

            # JSONファイルとして出力
            output_path = os.path.join(dir_path, filename)
            if os.path.exists(output_path):
                continue
            print(output_path, json_data)
            with open(output_path, 'w', encoding='utf-8') as jsonfile:
                json.dump(json_data, jsonfile, ensure_ascii=False, indent=2)


# 使用例
if __name__ == "__main__":
    csv_to_json("./original/station_hakodate_bus_03.csv")

