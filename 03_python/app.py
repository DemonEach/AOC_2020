def solution(text):
    c_row = 0
    c_col = 0
    tree_count = 0

    while c_row < len(text) - 1 and c_col < len(text[c_row]) - 3:
        c_row = c_row + 1
        c_col = c_col + 3

        if text[c_row][c_col] == '#':
            tree_count = tree_count + 1

    print(f'Final tree count: {tree_count}')


def extend_list(incoming_list):
    list_len = round(len(incoming_list) / 3)
    print(f'Number of lines: {list_len}')
    for i in range(len(incoming_list)):
        incoming_list[i] = incoming_list[i].replace('\n', '')
        incoming_list[i] = list_len * incoming_list[i]

if __name__ == "__main__":
    text = open('./sample-01.txt', 'r').readlines()
    extend_list(text)
    solution(text)