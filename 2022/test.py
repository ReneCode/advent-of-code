import numpy as np

factory = np.array([[1,0,0,0],[0,0,0,0]], dtype=np.int16)
factories = np.array([factory])

# print(factory)
f = factory
f[1][2] = 5
print(factories)
for i in range(500000):
  f[0][2] = 5
  factories = np.append(factories, [f], axis=0)
print(factories)
print(factories.shape)