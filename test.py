class MatrixFibonacci:
    Q = [[1, 1],
         [1, 0]]

    def __init__(self):
        self.__memo = {}

    def __multiply_matrices(self, M1, M2):
        a11 = M1[0][0]*M2[0][0] + M1[0][1]*M2[1][0]
        a12 = M1[0][0]*M2[0][1] + M1[0][1]*M2[1][1]
        a21 = M1[1][0]*M2[0][0] + M1[1][1]*M2[1][0]
        a22 = M1[1][0]*M2[0][1] + M1[1][1]*M2[1][1]
        r = [[a11, a12], [a21, a22]]
        return r

    def __get_matrix_power(self, M, p):
        if p == 1:
            return M
        if p in self.__memo:
            return self.__memo[p]
        K = self.__get_matrix_power(M, int(p/2))
        R = self.__multiply_matrices(K, K)
        self.__memo[p] = R
        return R

    def get_number(self, n):
        if n == 0:
            return 0
        if n == 1:
            return 1
        powers = [int(pow(2, b))
                  for (b, d) in enumerate(reversed(bin(n-1)[2:])) if d == '1']

        matrices = [self.__get_matrix_power(MatrixFibonacci.Q, p)
                    for p in powers]
        print(n, powers, matrices)
        # exit()

        while len(matrices) > 1:
            M1 = matrices.pop()
            M2 = matrices.pop()
            R = self.__multiply_matrices(M1, M2)
            matrices.append(R)
        return matrices[0][0][0]
def len_int (x):
    if x == 0:
        return 1
    len_ = 0
    while x:
        x //= 10
        len_  += 1
    return (len_)
import time
start_time = time.time()

mfib = MatrixFibonacci()
for n in range(0, 8192):
    num = mfib.get_number(n)
    if len_int(num) == 10:
        print(num)
        print(n)
        break;
print("--- %s seconds ---" % (time.time() - start_time))
