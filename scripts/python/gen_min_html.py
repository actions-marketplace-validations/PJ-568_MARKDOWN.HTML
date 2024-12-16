from htmlmin import minify
import sys

def conpress(file_path, target_path):
    # 读取 HTML 文件
    with open(file_path, 'r', encoding='utf-8') as f:
        # 写入压缩后的 HTML 文件
        with open(target_path, 'w', encoding='utf-8') as t:
            t.write(minify(f.read(), remove_comments=True, remove_empty_space=True))

if __name__ == "__main__":
    """
    压缩指定的 HTML 文件。

    :param path/to/file: 欲读取的源文件路径
    :param path/to/file: 欲写入的目标文件路径
    """
    if len(sys.argv) != 3:
        print("使用方法：python gen_min_html.py <源文件路径> <目标文件路径>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    target_path = sys.argv[2]
    conpress(file_path, target_path)