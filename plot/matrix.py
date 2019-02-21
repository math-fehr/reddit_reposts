from sklearn.decomposition import TruncatedSVD
from sklearn.random_projection import sparse_random_matrix
from scipy.sparse import csr_matrix
import matplotlib.pyplot as plt
from sklearn.manifold import TSNE

def read_ppmi(filepath):
    f = open(filepath, "r")
    line = f.readline()
    n_subreddits = int(line)
    subreddits = f.readline().split()
    is_nsfw = f.readline().split()
    kernel = []
    for line in f:
        line_parsed = line.split()
        kernel_line = []
        for elem in line_parsed:
            kernel_line.append(float(elem))
        kernel.append(kernel_line)
    return (kernel, subreddits, is_nsfw)

def main_ppmi():
    (X, subreddits, is_nsfw) = read_ppmi("kernel")
    X = TSNE(n_components=2, perplexity=5, learning_rate=10).fit_transform(X)
    for i, l in enumerate(X):
        if int(is_nsfw[i]) == 1:
            color = 'red'
        else:
            color = 'blue'
        plt.scatter(X[i,0], X[i,1], marker='x', color=color)
        plt.text(X[i,0]+0.3, X[i,1]+0.3, subreddits[i], fontsize=9)
    plt.show()

main_ppmi()
