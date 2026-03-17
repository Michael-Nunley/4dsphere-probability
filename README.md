# 4dsphere-probability
Calculates the probability that the sum of the squares of four random f64 numbers is less than or equal to one.

$\forall x,y,z,w \in \mathbb{R}, P\left(x^2 + y^2 + z^2 + w^2 \leq 1\right)$

In this video, https://www.youtube.com/watch?v=fsLh-NYhOoU , Grant invites the audience to make code to test if this is indeed the case. So I did.

With 15 threads, running one million times each, I got a decent result. It took a while though. This could be optimized, but why bother? This is not a useful thing to calculate in this way.
```
$ rustup run stable cargo run
   Compiling probability4d v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/probability4d`
0.30842878673333335 
```

$\frac{\pi^2}{2^5} - 0.30842878673333335 = -0.00000364919929089316...$
