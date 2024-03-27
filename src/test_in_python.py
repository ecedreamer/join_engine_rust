import csv
import time


def nested_loop_join(dataset_1, dataset_2, join_conditions):
    result = []

    for data_1 in dataset_1:
        for data_2 in dataset_2:
            if data_1.get(join_conditions[0]) == data_2.get(join_conditions[1]):
                result.append((data_1, data_2))

    return result

def sort_merge_join(dataset_1, dataset_2, join_conditions):
    sorted_dataset_1 = sorted(dataset_1, key=lambda x: x[join_conditions[0]])
    sorted_dataset_2 = sorted(dataset_2, key=lambda x: x[join_conditions[1]])

    result = []
    i = j = 0

    while i < len(sorted_dataset_1) and j < len(sorted_dataset_2):
        if sorted_dataset_1[i][join_conditions[0]] == sorted_dataset_2[j][join_conditions[1]]:
            result.append((sorted_dataset_1[i], sorted_dataset_2[j]))
            j += 1
        elif sorted_dataset_1[i][join_conditions[0]] < sorted_dataset_2[j][join_conditions[1]]:
            i += 1
        else:
            j += 1

    return result


def hash_join(dataset_1, dataset_2, join_conditions):
    # Create a hash table using dataset_1 data
    dataset_1_hash = {data_1[join_conditions[0]]: data_1 for data_1 in dataset_1}

    result = []

    # Probe the hash table with dataset_2 data
    for data_2 in dataset_2:
        data_1_match = dataset_1_hash.get(data_2[join_conditions[1]])
        if data_1_match:
            result.append((data_1_match, data_2))

    return result

def load_dataset(file_path: str):
    with open(file_path, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            yield row


def main():

    products = list(load_dataset("src/datasets/olist_products_dataset.csv"))
    order_items = list(load_dataset("src/datasets/olist_order_items_dataset.csv"))

    join_conditions = ("product_id", "product_id")

    t1 = time.time()
    result1 = sort_merge_join(products, order_items, join_conditions)
    t2 = time.time()
    result2 = hash_join(products, order_items, join_conditions)
    t3 = time.time()
    result3 = nested_loop_join(products, order_items, join_conditions)
    t4 = time.time()

    print(len(result1), len(result2), len(result3))

    print("Sort Merge Join:", t2-t1, "Hash Join", t3-t2, "Nested Loop Join", t4-t3)
    assert len(result1) == len(result2) == len(result3)


if __name__ == '__main__':
    main()
