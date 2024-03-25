import time
import random
import string

used_ids = set()

def generate_id():
    while True:
        id_candidate = ''.join(random.choices(string.ascii_lowercase, k=3))
        if id_candidate not in used_ids:
            used_ids.add(id_candidate)
            return id_candidate

def generate_name():
    return ''.join(random.choices(string.ascii_uppercase, k=3))

def generate_dataset(count, gender):
    dataset = []
    for _ in range(count):
        record = {
            "name": generate_name(),
            "age": random.randint(25, 35),
            "id": generate_id(),
            "gender": gender
        }
        dataset.append(record)
    return dataset


def nested_loop_join(mens, womens):
    result = []

    for men in mens:
        for women in womens:
            if men.get("id") == women.get("id"):
                result.append((men, women))

    return result

def sort_merge_join(mens, womens):
    sorted_mens = sorted(mens, key=lambda x: x["id"])
    sorted_womens = sorted(womens, key=lambda x: x["id"])

    result = []
    i = j = 0

    while i < len(sorted_mens) and j < len(sorted_womens):
        if sorted_mens[i]["id"] == sorted_womens[j]["id"]:
            result.append((sorted_mens[i], sorted_womens[j]))
            j += 1
        elif sorted_mens[i]["id"] < sorted_womens[j]["id"]:
            i += 1
        else:
            j += 1

    return result


def hash_join(mens, womens):
    # Create a hash table using mens data
    mens_hash = {men['id']: men for men in mens}

    result = []

    # Probe the hash table with womens data
    for women in womens:
        men_match = mens_hash.get(women['id'])
        if men_match:
            result.append((men_match, women))

    return result


def main():
    global used_ids
    mens = generate_dataset(10000, "mens")
    used_ids = set()
    womens = generate_dataset(10000, "womens")
    t1 = time.time()
    result1 = sort_merge_join(mens, womens)
    t2 = time.time()
    result2 = hash_join(mens, womens)
    t3 = time.time()
    result3 = nested_loop_join(mens, womens)
    t4 = time.time()

    print(result1, "\n", result2, "\n", result3)
    print(len(result1), len(result2), len(result3))

    print("Sort Merge Join:", t2-t1, "Hash Join", t3-t2, "Nested Loop Join", t4-t3)
    assert len(result1) == len(result2) == len(result3)


if __name__ == '__main__':
    main()
