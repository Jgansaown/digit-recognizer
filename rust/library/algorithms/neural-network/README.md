# Neural Network Crate

http://neuralnetworksanddeeplearning.com/


## The Four Fundamental Equations for Back Propagation

http://neuralnetworksanddeeplearning.com/chap2.html#the_four_fundamental_equations_behind_backpropagation

### 1. Error in output layer $L$

$$
\delta_j^L = \frac{\partial C}{\partial a_j^L} \sigma'(z_j^L)$$

### 2. Error in layer $l$ in terms of the error in the next layer, $l+1$

$$
\delta^l = ((w^{l+1})^T \delta^{l+1}) \odot \sigma'(z^l)
$$

### 3. Rate of change of the cost with respect to any bias in the network

$$
\frac{\partial C}{\partial b_j^l} = \delta_j^l
$$

### 4. Rate of change of the cost with respect to any weight in the network

$$
\frac{\partial C}{\partial w_{jk}^l} = a_k^{l-1}\delta_j^l
$$

## Implementation of Back Propagation in Python

## Implementation of Back Propagation in Rust