import itertools


def check_adj(r: list[int], a: int, b: int, diff_sign: bool | None = None) -> bool:
    return 0 < abs(r[a] - r[b]) <= 3 and (diff_sign is None or (r[a] - r[b] > 0) == diff_sign)


def is_report_safe(r: list[int]) -> bool:
    if all(check_adj(r, i-1, i, r[0] - r[1] > 0) for i in range(1, len(r))):
        return True
    else:
        for removed in range(len(r)):
            if removed == 0:
                diff = r[1] - r[2] > 0
            elif removed == 1:
                diff = r[0] - r[2] > 0
            else:
                diff = r[0] - r[1] > 0
            if all(check_adj(r, i, j, diff) for i, j in itertools.pairwise(x for x in range(len(r)) if x != removed)):
                return True
        return False


if __name__ == '__main__':
    with open('../reports.txt') as f:
        reports: list[list[int]] = [[int(x) for x in line.strip().split()] for line in f]

    safe_reports = sum(map(is_report_safe, reports))

    print(safe_reports)

