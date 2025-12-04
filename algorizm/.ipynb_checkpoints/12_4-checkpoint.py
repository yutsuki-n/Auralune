# def S(n):
#     return n**2

# def search(num_List, x):
#     for i in range(num_List):
#         if num_List[i] == x:
#             return  i, num_List


# def Bo2(a,b):
#     for i in range(a, b-1):
#         if 100 // i == 0:
#             return "Yes"
#     return "no"

# def A03( k, P, Q):
#     for p_i in range(len(P)):
#         for q_i in range(len(Q)):
#             if P[p_i] + Q[q_i] == k:
#                 return True
#     return False


# def B03(A):
#     for i in range(len(A) - 1):
#         for j in range(i, len(A)-1):
#             for k in range(j, len(A) - 1):
#                 if A[i] + A[j] + A[k] == 1000:
#                     return True
#     return False 

# def A04(N):
#     binary = []
#     for i in range(9):
#         if N >= 2**(9 - i) & N <= 2**(10-i):
#             binary.append(1)
#         else:
#             binary.append(0)
#     return  binary

def linear_search(arr, target):
    for i, v in enumerate(arr):
        if v == target:
            return i
    return -1

arr = []
for i in range(100000):
    arr.append(i)

linear_search(arr, 99999)