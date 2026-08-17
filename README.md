# Fórmulas Matemáticas del Proyecto Matrix

Este archivo reúne todas las fórmulas matemáticas utilizadas en los ejercicios del proyecto **Enter the Matrix: An introduction to Linear Algebra**, junto con ejemplos numéricos para ilustrar cada concepto.

---

## Índice

1. [Ejercicio 00 - Sumar, Restar y Escalar](#ejercicio-00---sumar-restar-y-escalar)
2. [Ejercicio 01 - Combinación Lineal](#ejercicio-01---combinación-lineal)
3. [Ejercicio 02 - Interpolación Lineal](#ejercicio-02---interpolación-lineal)
4. [Ejercicio 03 - Producto Escalar](#ejercicio-03---producto-escalar)
5. [Ejercicio 04 - Norma](#ejercicio-04---norma)
6. [Ejercicio 05 - Coseno](#ejercicio-05---coseno)
7. [Ejercicio 06 - Producto Vectorial](#ejercicio-06---producto-vectorial)
8. [Ejercicio 07 - Multiplicación de Matriz-Vector y Matriz-Matriz](#ejercicio-07---multiplicación-de-matriz-vector-y-matriz-matriz)
9. [Ejercicio 08 - Traza](#ejercicio-08---traza)
10. [Ejercicio 09 - Traspuesta](#ejercicio-09---traspuesta)
11. [Ejercicio 10 - Forma Escalonada por Filas](#ejercicio-10---forma-escalonada-por-filas)
12. [Ejercicio 11 - Determinante](#ejercicio-11---determinante)
13. [Ejercicio 12 - Inversa](#ejercicio-12---inversa)
14. [Ejercicio 13 - Rango](#ejercicio-13---rango)

---

## Ejercicio 00 - Sumar, Restar y Escalar

### 1. Suma de Vectores y Matrices
Suma elemento a elemento para objetos de la misma dimensión.

* **Vectores ($u, v \in \mathbb{K}^n$):**
  $$u + v = \begin{bmatrix} u_1 + v_1 \\ u_2 + v_2 \\ \vdots \\ u_n + v_n \end{bmatrix}$$
  * **Ejemplo:**
    $$u = \begin{bmatrix} 2 \\ 3 \end{bmatrix}, \, v = \begin{bmatrix} 5 \\ 7 \end{bmatrix} \implies u + v = \begin{bmatrix} 2 + 5 \\ 3 + 7 \end{bmatrix} = \begin{bmatrix} 7 \\ 10 \end{bmatrix}$$

* **Matrices ($A, B \in \mathbb{K}^{m \times n}$):**
  $$(A + B)_{ij} = A_{ij} + B_{ij}$$
  * **Ejemplo:**
    $$A = \begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}, \, B = \begin{bmatrix} 7 & 4 \\ -2 & 2 \end{bmatrix} \implies A + B = \begin{bmatrix} 1+7 & 2+4 \\ 3-2 & 4+2 \end{bmatrix} = \begin{bmatrix} 8 & 6 \\ 1 & 6 \end{bmatrix}$$

### 2. Resta de Vectores y Matrices
Resta elemento a elemento para objetos de la misma dimensión.

* **Vectores ($u, v \in \mathbb{K}^n$):**
  $$u - v = \begin{bmatrix} u_1 - v_1 \\ u_2 - v_2 \\ \vdots \\ u_n - v_n \end{bmatrix}$$
  * **Ejemplo:**
    $$u = \begin{bmatrix} 2 \\ 3 \end{bmatrix}, \, v = \begin{bmatrix} 5 \\ 7 \end{bmatrix} \implies u - v = \begin{bmatrix} 2 - 5 \\ 3 - 7 \end{bmatrix} = \begin{bmatrix} -3 \\ -4 \end{bmatrix}$$

* **Matrices ($A, B \in \mathbb{K}^{m \times n}$):**
  $$(A - B)_{ij} = A_{ij} - B_{ij}$$
  * **Ejemplo:**
    $$A = \begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}, \, B = \begin{bmatrix} 7 & 4 \\ -2 & 2 \end{bmatrix} \implies A - B = \begin{bmatrix} 1-7 & 2-4 \\ 3-(-2) & 4-2 \end{bmatrix} = \begin{bmatrix} -6 & -2 \\ 5 & 2 \end{bmatrix}$$

### 3. Escalamiento (Multiplicación por un Escalar)
Multiplica cada elemento por un escalar $a \in \mathbb{K}$.

* **Vectores ($u \in \mathbb{K}^n, a \in \mathbb{K}$):**
  $$a \cdot u = \begin{bmatrix} a \cdot u_1 \\ a \cdot u_2 \\ \vdots \\ a \cdot u_n \end{bmatrix}$$
  * **Ejemplo:**
    $$u = \begin{bmatrix} 2 \\ 3 \end{bmatrix}, \, a = 2 \implies 2 \cdot u = \begin{bmatrix} 2 \cdot 2 \\ 2 \cdot 3 \end{bmatrix} = \begin{bmatrix} 4 \\ 6 \end{bmatrix}$$

* **Matrices ($A \in \mathbb{K}^{m \times n}, a \in \mathbb{K}$):**
  $$(a \cdot A)_{ij} = a \cdot A_{ij}$$
  * **Ejemplo:**
    $$A = \begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}, \, a = 2 \implies 2 \cdot A = \begin{bmatrix} 2 \cdot 1 & 2 \cdot 2 \\ 2 \cdot 3 & 2 \cdot 4 \end{bmatrix} = \begin{bmatrix} 2 & 4 \\ 6 & 8 \end{bmatrix}$$

---

## Ejercicio 01 - Combinación Lineal

Para una lista de vectores $u = (u_1, u_2, \dots, u_k)$ y coeficientes escalares $\lambda = (\lambda_1, \lambda_2, \dots, \lambda_k) \in \mathbb{K}^k$:

$$v = \sum_{i=1}^k \lambda_i u_i = \lambda_1 u_1 + \lambda_2 u_2 + \dots + \lambda_k u_k$$

* **Ejemplo:**
  Dados los vectores $u_1 = \begin{bmatrix} 1 \\ 2 \\ 3 \end{bmatrix}$, $u_2 = \begin{bmatrix} 0 \\ 10 \\ -100 \end{bmatrix}$ y coeficientes $\lambda_1 = 10$, $\lambda_2 = -2$:
  $$v = 10 \begin{bmatrix} 1 \\ 2 \\ 3 \end{bmatrix} + (-2) \begin{bmatrix} 0 \\ 10 \\ -100 \end{bmatrix} = \begin{bmatrix} 10 \cdot 1 + 0 \\ 10 \cdot 2 - 20 \\ 10 \cdot 3 + 200 \end{bmatrix} = \begin{bmatrix} 10 \\ 0 \\ 230 \end{bmatrix}$$

---

## Ejercicio 02 - Interpolación Lineal

Dados dos objetos (escalares, vectores o matrices) $u, v$ del mismo espacio vectorial, y un parámetro real $t$:

$$\mathrm{lerp}(u, v, t) = u + t(v - u) = (1 - t)u + t v$$

* **Ejemplo:**
  Sean $u = \begin{bmatrix} 2 \\ 1 \end{bmatrix}$, $v = \begin{bmatrix} 4 \\ 2 \end{bmatrix}$ y $t = 0.3$:
  $$\mathrm{lerp}(u, v, 0.3) = \begin{bmatrix} 2 \\ 1 \end{bmatrix} + 0.3 \left( \begin{bmatrix} 4 \\ 2 \end{bmatrix} - \begin{bmatrix} 2 \\ 1 \end{bmatrix} \right) = \begin{bmatrix} 2 \\ 1 \end{bmatrix} + 0.3 \begin{bmatrix} 2 \\ 1 \end{bmatrix} = \begin{bmatrix} 2.6 \\ 1.3 \end{bmatrix}$$

---

## Ejercicio 03 - Producto Escalar

El producto escalar (o producto interno) de dos vectores $u, v \in \mathbb{K}^n$:

$$\langle u, v \rangle = u \cdot v = \sum_{i=1}^n u_i v_i = u_1 v_1 + u_2 v_2 + \dots + u_n v_n$$

* **Ejemplo:**
  Sean $u = \begin{bmatrix} -1 \\ 6 \end{bmatrix}$ y $v = \begin{bmatrix} 3 \\ 2 \end{bmatrix}$:
  $$u \cdot v = (-1 \cdot 3) + (6 \cdot 2) = -3 + 12 = 9$$

---

## Ejercicio 04 - Norma

Para un vector $u \in \mathbb{R}^n$, se definen tres normas distintas:

### 1. Norma $l_1$ (Norma de Manhattan o del Taxista)
* **Fórmula:**
  $$\|u\|_1 = \sum_{i=1}^n |u_i|$$
* **Definición:** Consiste en la suma de los valores absolutos de todas las componentes del vector.
* **Interpretación geométrica:** Representa la distancia que se recorrería entre dos puntos si el desplazamiento estuviera limitado a moverse únicamente en paralelo a los ejes coordenados.
* **Ejemplo:**
  $$u = \begin{bmatrix} 1 \\ -2 \\ 3 \end{bmatrix} \implies \|u\|_1 = |1| + |-2| + |3| = 6$$

### 2. Norma $l_2$ (Norma Euclidiana)
* **Fórmula:**
  $$\|u\|_2 = \sqrt{u \cdot u} = \sqrt{\sum_{i=1}^n u_i^2}$$
* **Definición:** Es la raíz cuadrada de la suma de los componentes elevados al cuadrado.
* **Interpretación geométrica:** Equivale a la distancia física real en línea recta desde el origen hasta el extremo del vector (su longitud).
* **Ejemplo:**
  $$u = \begin{bmatrix} 1 \\ -2 \\ 3 \end{bmatrix} \implies \|u\|_2 = \sqrt{1^2 + (-2)^2 + 3^2} = \sqrt{1 + 4 + 9} = \sqrt{14} \approx 3.74$$

### 3. Norma $l_\infty$ (Norma del Supremo o Uniforme)
* **Fórmula:**
  $$\|u\|_\infty = \max_{1 \le i \le n} (|u_i|)$$
* **Definición:** Es el mayor valor absoluto entre todas las coordenadas del vector.
* **Interpretación geométrica:** Mide la máxima distancia recorrida a lo largo de un único eje coordenado.
* **Ejemplo:**
  $$u = \begin{bmatrix} 1 \\ -2 \\ 3 \end{bmatrix} \implies \|u\|_\infty = \max(|1|, |-2|, |3|) = 3$$

---

## Ejercicio 05 - Coseno

El coseno del ángulo $\theta$ entre dos vectores no nulos $u, v \in \mathbb{R}^n$:

$$\cos(\theta) = \frac{u \cdot v}{\|u\|_2 \|v\|_2}$$

* **Ejemplo:**
  Sean $u = \begin{bmatrix} 1 \\ 2 \end{bmatrix}$ y $v = \begin{bmatrix} 2 \\ 4 \end{bmatrix}$ (vectores con la misma dirección y sentido):
  $$u \cdot v = (1 \cdot 2) + (2 \cdot 4) = 10$$
  $$\|u\|_2 = \sqrt{1^2 + 2^2} = \sqrt{5}, \quad \|v\|_2 = \sqrt{2^2 + 4^2} = \sqrt{20}$$
  $$\cos(\theta) = \frac{10}{\sqrt{5} \cdot \sqrt{20}} = \frac{10}{\sqrt{100}} = 1.0 \implies \theta = 0^\circ$$

---

## Ejercicio 06 - Producto Vectorial

Para dos vectores tridimensionales $u, v \in \mathbb{R}^3$, el producto vectorial $u \times v$ genera un vector perpendicular a ambos definido por:

$$u \times v = \begin{bmatrix} u_y v_z - u_z v_y \\ u_z v_x - u_x v_z \\ u_x v_y - u_y v_x \end{bmatrix}$$

* **Ejemplo:**
  Sean $u = \begin{bmatrix} 1 \\ 2 \\ 3 \end{bmatrix}$ y $v = \begin{bmatrix} 4 \\ 5 \\ 6 \end{bmatrix}$:
  $$u \times v = \begin{bmatrix} (2 \cdot 6) - (3 \cdot 5) \\ (3 \cdot 4) - (1 \cdot 6) \\ (1 \cdot 5) - (2 \cdot 4) \end{bmatrix} = \begin{bmatrix} 12 - 15 \\ 12 - 6 \\ 5 - 8 \end{bmatrix} = \begin{bmatrix} -3 \\ 6 \\ -3 \end{bmatrix}$$

---

## Ejercicio 07 - Multiplicación de Matriz-Vector y Matriz-Matriz

### 1. Multiplicación Matriz-Vector
Para una matriz $A \in \mathbb{K}^{m \times n}$ y un vector $u \in \mathbb{K}^n$, el resultado es un vector $v \in \mathbb{K}^m$ cuyas componentes son:

$$v_i = (A u)_i = \sum_{j=1}^n A_{ij} u_j \quad \text{para } i = 1, \dots, m$$

* **Representación visual:**
  Se multiplica cada fila de la matriz por el vector columna completo:
  $$\begin{bmatrix}
  \color{red}{A_{11}} & \color{red}{A_{12}} & \color{red}{\dots} & \color{red}{A_{1n}} \\
  A_{21} & A_{22} & \dots & A_{2n} \\
  \vdots & \vdots & \ddots & \vdots \\
  A_{m1} & A_{m2} & \dots & A_{mn}
  \end{bmatrix}
  \begin{bmatrix}
  \color{blue}{u_1} \\
  \color{blue}{u_2} \\
  \color{blue}{\vdots} \\
  \color{blue}{u_n}
  \end{bmatrix}
  =
  \begin{bmatrix}
  \color{purple}{A_{11}u_1 + A_{12}u_2 + \dots + A_{1n}u_n} \\
  A_{21}u_1 + A_{22}u_2 + \dots + A_{2n}u_n \\
  \vdots \\
  A_{m1}u_1 + A_{m2}u_2 + \dots + A_{mn}u_n
  \end{bmatrix}$$
* **Ejemplo:**
  $$A = \begin{bmatrix} 2 & 0 \\ 0 & 2 \end{bmatrix}, \, u = \begin{bmatrix} 4 \\ 2 \end{bmatrix} \implies A u = \begin{bmatrix} 2 \cdot 4 + 0 \cdot 2 \\ 0 \cdot 4 + 2 \cdot 2 \end{bmatrix} = \begin{bmatrix} 8 \\ 4 \end{bmatrix}$$

---

### 2. Multiplicación Matriz-Matriz
Para dos matrices $A \in \mathbb{K}^{m \times n}$ y $B \in \mathbb{K}^{n \times p}$, la multiplicación resulta en una matriz $C \in \mathbb{K}^{m \times p}$ cuyas entradas se calculan como:

$$C_{ij} = (A B)_{ij} = \sum_{k=1}^n A_{ik} B_{kj} \quad \text{para } i = 1, \dots, m \text{ y } j = 1, \dots, p$$

* **Representación visual (cálculo de la celda $C_{ij}$):**
  Se obtiene mediante el producto escalar de la fila $i$ de la matriz $A$ por la columna $j$ de la matriz $B$:
  $$\begin{bmatrix}
  \dots & \dots & & \dots \\
  \color{red}{A_{i1}} & \color{red}{A_{i2}} & \color{red}{\dots} & \color{red}{A_{in}} \\
  \dots & \dots & & \dots
  \end{bmatrix}
  \begin{bmatrix}
  \dots & \color{blue}{B_{1j}} & \dots \\
  \dots & \color{blue}{B_{2j}} & \dots \\
        & \color{blue}{\vdots} &       \\
  \dots & \color{blue}{B_{nv}} & \dots
  \end{bmatrix}
  =
  \begin{bmatrix}
  \ddots & \vdots & \\
  \dots & \color{purple}{C_{ij}} & \dots \\
        & \vdots & \ddots
  \end{bmatrix}$$
  $$\color{purple}{C_{ij}} = \color{red}{A_{i1}}\color{blue}{B_{1j}} + \color{red}{A_{i2}}\color{blue}{B_{2j}} + \dots + \color{red}{A_{in}}\color{blue}{B_{nj}}$$
* **Ejemplo:**
  $$A = \begin{bmatrix} 3 & -5 \\ 6 & 8 \end{bmatrix}, \, B = \begin{bmatrix} 2 & 1 \\ 4 & 2 \end{bmatrix}$$
  $$C = A B = \begin{bmatrix} (3 \cdot 2) + (-5 \cdot 4) & (3 \cdot 1) + (-5 \cdot 2) \\ (6 \cdot 2) + (8 \cdot 4) & (6 \cdot 1) + (8 \cdot 2) \end{bmatrix} = \begin{bmatrix} -14 & -7 \\ 44 & 22 \end{bmatrix}$$

---

## Ejercicio 08 - Traza

La traza de una matriz cuadrada $A \in \mathbb{K}^{n \times n}$ es la suma de los elementos de su diagonal principal:

$$\mathrm{Tr}(A) = \sum_{i=1}^n A_{ii}$$

* **Ejemplo:**
  $$A = \begin{bmatrix} 2 & -5 & 0 \\ 4 & 3 & 7 \\ -2 & 3 & 4 \end{bmatrix} \implies \mathrm{Tr}(A) = 2 + 3 + 4 = 9$$

---

## Ejercicio 09 - Traspuesta

La traspuesta de una matriz $A \in \mathbb{K}^{m \times n}$ es una matriz $A^T \in \mathbb{K}^{n \times m}$ definida por:

$$(A^T)_{ij} = A_{ji}$$

* **Ejemplo:**
  $$A = \begin{bmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \end{bmatrix} \implies A^T = \begin{bmatrix} 1 & 4 \\ 2 & 5 \\ 3 & 6 \end{bmatrix}$$

---

## Ejercicio 10 - Forma Escalonada por Filas

Consiste en transformar una matriz $A \in \mathbb{K}^{m \times n}$ a su forma escalonada (generalmente escalonada reducida o RREF) mediante la aplicación de operaciones elementales de fila (eliminación de Gauss-Jordan).

* **Ejemplo:**
  Dada la matriz $A = \begin{bmatrix} 1 & 2 \\ 2 & 4 \end{bmatrix}$:
  1. Restamos dos veces la fila 1 a la fila 2 ($R_2 \leftarrow R_2 - 2R_1$):
     $$\begin{bmatrix} 1 & 2 \\ 2 - 2(1) & 4 - 2(2) \end{bmatrix} = \begin{bmatrix} 1 & 2 \\ 0 & 0 \end{bmatrix}$$
  La forma escalonada por filas de $A$ es:
  $$\mathrm{RREF}(A) = \begin{bmatrix} 1 & 2 \\ 0 & 0 \end{bmatrix}$$

---

## Ejercicio 11 - Determinante

Para una matriz cuadrada $A \in \mathbb{K}^{n \times n}$ (con $n \le 4$), el determinante se puede calcular reduciendo la matriz a una forma triangular superior $A'$ mediante operaciones de fila (llevando el control de los intercambios de fila $s$):

$$\det(A) = (-1)^s \prod_{i=1}^n A'_{ii}$$

* **Ejemplo:**
  Para una matriz ya triangular (o diagonal) $A = \begin{bmatrix} 2 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 2 \end{bmatrix}$:
  $$\det(A) = 2 \cdot 2 \cdot 2 = 8$$

---

## Ejercicio 12 - Inversa

La inversa de una matriz cuadrada $A \in \mathbb{K}^{n \times n}$, si existe, es la única matriz $A^{-1}$ tal que:

$$A \cdot A^{-1} = A^{-1} \cdot A = I_n$$

Se puede calcular mediante el algoritmo de Gauss-Jordan sobre la matriz aumentada $[A \mid I_n]$:

$$[A \mid I_n] \xrightarrow{\text{Operaciones elementales de fila}} [I_n \mid A^{-1}]$$

* **Ejemplo:**
  Para $A = \begin{bmatrix} 2 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 2 \end{bmatrix}$:
  Su inversa es:
  $$A^{-1} = \begin{bmatrix} 0.5 & 0 & 0 \\ 0 & 0.5 & 0 \\ 0 & 0 & 0.5 \end{bmatrix}$$
  Debido a que:
  $$\begin{bmatrix} 2 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 2 \end{bmatrix} \begin{bmatrix} 0.5 & 0 & 0 \\ 0 & 0.5 & 0 \\ 0 & 0 & 0.5 \end{bmatrix} = \begin{bmatrix} 1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1 \end{bmatrix} = I_3$$

---

## Ejercicio 13 - Rango

El rango de una matriz $A \in \mathbb{K}^{m \times n}$ es la cantidad de filas o columnas linealmente independientes, lo cual es equivalente a la cantidad de filas no nulas en su forma escalonada reducida por filas (RREF).

* **Ejemplo:**
  Dada la matriz $A = \begin{bmatrix} 1 & 2 & 0 & 0 \\ 2 & 4 & 0 & 0 \\ -1 & 2 & 1 & 1 \end{bmatrix}$:
  Su forma reducida por filas (RREF) es:
  $$\mathrm{RREF}(A) = \begin{bmatrix} 1 & 0 & -0.5 & -0.5 \\ 0 & 1 & 0.25 & 0.25 \\ 0 & 0 & 0 & 0 \end{bmatrix}$$
  Como hay exactamente $2$ filas que no son completamente nulas, el rango es:
  $$\mathrm{rank}(A) = 2$$
