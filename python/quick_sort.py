def quick_sort(seq):
    """
    quick sort python implementation
    :param seq: A list of integers
    :rtype: A list of sorted integers
    """
    if len(seq) <= 1:
        return seq
    else:
        pivot = seq[0]
        left, right = [], []
        for x in seq[1:]:
            if x < pivot:
                left.append(x)
            else:
                right.append(x)
        return quick_sort(left) + [pivot] + quick_sort(right)