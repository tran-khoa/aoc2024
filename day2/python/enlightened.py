from naive import is_report_safe as is_report_safe_ref


def check_adj(r: list[int], a: int, b: int, diff_sign: bool | None = None) -> bool:
    return 0 < abs(r[a] - r[b]) <= 3 and (
        diff_sign is None or (r[a] - r[b] > 0) == diff_sign
    )


def is_report_safe(r: list[int]) -> bool:
    if len(r) <= 2:
        return True
    if len(r) == 3:
        return check_adj(r, 0, 1) or check_adj(r, 1, 2) or check_adj(r, 0, 2)

    used_fix = False

    # Check for early monotony flip
    pair1_diff = r[0] - r[1] > 0
    pair2_diff = r[1] - r[2] > 0
    pair3_diff = r[2] - r[3] > 0
    target_diff = sum([pair1_diff, pair2_diff, pair3_diff]) >= 2

    if not check_adj(r, 0, 1, target_diff):
        used_fix = True
        target_diff_sign = target_diff
        if not 0 < abs(r[2] - r[3]) <= 3:
            # at least 2 errors, can't recover
            return False
        if check_adj(r, 0, 2, target_diff_sign):
            diff_sign = r[0] - r[2] > 0
            r = [r[0], *r[2:]]
        elif check_adj(r, 1, 2, target_diff_sign):
            diff_sign = r[1] - r[2] > 0
            r = r[1:]
        else:
            return False
    else:
        diff_sign = r[0] - r[1] > 0

    assert r[0] != r[1]
    assert check_adj(r, 0, 1, diff_sign)

    for i in range(1, len(r) - 1):
        if check_adj(r, i - 1, i, diff_sign):
            continue
        elif not used_fix:
            used_fix = True
            if not (
                check_adj(r, i - 2, i, diff_sign) and check_adj(r, i, i + 1, diff_sign)
            ):
                if check_adj(r, i - 1, i + 1, diff_sign):
                    r[i] = r[i - 1]
                else:
                    return False
        else:
            return False

    return check_adj(r, len(r) - 2, len(r) - 1, diff_sign) or (not used_fix)


with open("tests.txt") as f:
    tests = [
        (line.strip().split()[0] == "T", [int(x) for x in line.strip().split()[1:]])
        for line in f
    ]
    for expected, r in tests:
        assert is_report_safe(r) == expected, f"{r} failed, should be {expected}"
        print(f"{r} passed")

if __name__ == "__main__":
    with open("reports.txt") as f:
        reports: list[list[int]] = [
            [int(x) for x in line.strip().split()] for line in f
        ]

    for r in reports:
        assert is_report_safe(r) == is_report_safe_ref(
            r
        ), f"{r} failed, should be {is_report_safe_ref(r)}"
    safe_reports = sum(map(is_report_safe, reports))

    print(safe_reports)
