#%%
import tensorflow as tf

#%%
x = tf.random.uniform([3,3])
print(x)
print('gpus', tf.config.experimental.list_physical_devices('GPU'))
print('is on GPU?', x.device.endswith('GPU:0'))

# %%
import time

def timed_matmut(x):
    start = time.time()
    for i in range(10):
        tf.matmul(x, x)
    duration = time.time() - start
    print(f"10 loops: {(duration*1000):.2f}ms")

def time_comparison():
    print("on CPU")
    with tf.device("CPU:0"):
        x = tf.random.uniform([1000, 1000])
        print(x.device)
        assert(x.device.endswith("CPU:0"))
        timed_matmut(x)

    print("on GPU")
    with tf.device("GPU:0"):
        x = tf.random.uniform([1000, 1000])
        print(x.device)
        assert(x.device.endswith("GPU:0"))
        timed_matmut(x)

time_comparison()

# %%

class TestLayer(tf.keras.layers.Layers):
    def __init__(self, num_outputs):
        super(TestLayer, self).__init__()
        self.num_outputs = num_outputs

    def build(self, input_shape):
        self.kernel = self.add_weight("kernel",
            shape=[int(input_shape[-1]), self.num_outputs]
        )
    
    def call(self, input, training=False):
        return tf.matmul(input, self.kernel)

#%%
import sys
sys.path.append('.')
sys.path.append('../')
sys.path.append('../../')

#%%
from board.board import Board

def board_2_graph(Board):
    print('ok')

# %%
