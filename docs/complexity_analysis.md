# Análisis de Complejidad Temporal y Espacial

Este documento evalúa cada una de las funciones implementadas en la biblioteca de álgebra lineal frente a los límites de complejidad exigidos por el tema del proyecto (*subject*).

## ¿Qué es la Complejidad Temporal y Espacial?

En ciencias de la computación, la **complejidad** de un algoritmo describe la cantidad de recursos (tiempo de procesamiento o memoria) que este consume a medida que el tamaño de los datos de entrada ($n$) crece. Para representarla se utiliza la **notación Big O (O grande)**, que describe el límite superior del peor escenario de ejecución.

### 1. Complejidad Temporal
Mide cómo escala el **tiempo de ejecución** (o el número de operaciones básicas como sumas, multiplicaciones y accesos a memoria) en función del tamaño de la entrada.
* **$O(1)$ (Constante):** El algoritmo tarda lo mismo sin importar el tamaño de los datos (por ejemplo, acceder a un elemento en un array por su índice o realizar el producto cruzado de 3D).
* **$O(n)$ (Lineal):** El número de operaciones crece de manera proporcional al tamaño de los datos (por ejemplo, recorrer un vector de tamaño $n$ para calcular su norma o su suma).
* **$O(n^3)$ (Cúbico):** El número de operaciones crece de forma cúbica respecto al tamaño. Es común en operaciones de matrices como la eliminación Gaussiana, donde hay tres bucles anidados que dependen de las dimensiones de la matriz.

### 2. Complejidad Espacial
Mide la cantidad de **memoria adicional (auxiliar)** que el algoritmo necesita para ejecutarse, además de los datos de entrada ya provistos.
* **$O(1)$ (Espacio auxiliar constante):** El algoritmo opera modificando las variables existentes *en el lugar* (in-place) sin reservar memoria nueva (por ejemplo, sumar dos vectores modificando la estructura original).
* **$O(n)$ o $O(n^2)$ (Espacio auxiliar lineal o cuadrático):** El algoritmo crea copias de los vectores o matrices de tamaño $n$ o $n^2$ para realizar cálculos intermedios o para devolver un nuevo objeto resultante.

---

## Tabla Resumen de Complejidad

A continuación, se detalla el análisis para un vector de dimensión $n$ o matrices de dimensiones $m \times n$ (para matrices cuadradas, consideramos la dimensión como $n \times n$).

| Ejercicio | Función / Operación | Complejidad Requerida (Tiempo) | Complejidad Real (Tiempo) | Complejidad Requerida (Espacio) | Complejidad Real (Espacio) | ¿Cumple? |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **00** | Suma de Vectores | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **00** | Resta de Vectores | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **00** | Escalar Vectores | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **00** | Suma de Matrices | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **00** | Resta de Matrices | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **00** | Escalar Matrices | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **01** | Combinación Lineal | $O(n)$ | **$O(k \cdot n)$** | $O(n)$ | **$O(n)$** | **Sí** |
| **02** | Interpolación Lineal | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(n)$** | **Sí** |
| **03** | Producto Escalar | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **04** | Normas ($l_1, l_2, l_\infty$) | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **05** | Coseno del Ángulo | $O(n)$ | **$O(n)$** | $O(n)$ | **$O(1)$** (Auxiliar) / **$O(n)$** (Total) | **Sí** |
| **06** | Producto Vectorial | N/A | **$O(1)$** | N/A | **$O(1)$** | **Sí** |
| **07** | Matriz-Vector ($A u$) | $O(mn)$ | **$O(mn)$** | $O(mn)$ | **$O(m)$** (Auxiliar) / **$O(mn)$** (Total) | **Sí** |
| **07** | Matriz-Matriz ($A B$) | $O(mnp)$ | **$O(mnp)$** | $O(mn + mp + np)$ | **$O(mp)$** (Auxiliar) / **$O(mn+mp+np)$** | **Sí** |
| **08** | Traza de Matriz | $O(n)$ | **$O(n)$** | N/A | **$O(1)$** | **Sí** |
| **09** | Traspuesta de Matriz | $O(mn)$ | **$O(mn)$** | $O(mn)$ | **$O(mn)$** | **Sí** |
| **10** | Forma Escalonada | $O(n^3)$ | **$O(m \cdot n^2)$** | $O(n^2)$ | **$O(mn)$** | **Sí** |
| **11** | Determinante | $O(n^3)$ | **$O(n^3)$** | $O(n^2)$ | **$O(n^2)$** | **Sí** |
| **12** | Inversa de Matriz | $O(n^3)$ | **$O(n^3)$** | $O(n^2)$ | **$O(n^2)$** | **Sí** |
| **13** | Rango de Matriz | $O(n^3)$ | **$O(n^3)$** | N/A | **$O(n^2)$** | **Sí** |

> [!NOTE]
> * **$n$** representa el número total de coordenadas en vectores/matrices (por ejemplo, para una matriz de $m \times p$, el total de coordenadas es $m \cdot p$).
> * **$k$** en la Combinación Lineal representa el número de vectores que se combinan. La complejidad es lineal respecto al tamaño total de la entrada $O(k \cdot n)$, lo cual es el óptimo teórico.

---

## Análisis Detallado por Ejercicio

### Ejercicio 00: Suma, Resta y Escalar (Vectores y Matrices)
* **Complejidad Temporal:** Las funciones `add`, `sub` y `scl` utilizan un único bucle indexado (o dos en el caso de matrices de dos dimensiones `Vec<Vec<K>>`) que recorre todos los elementos secuencialmente. Esto realiza exactamente una operación aritmética básica por coordenada. Por lo tanto, es **$O(n)$**.
* **Complejidad Espacial:** Las mutaciones se realizan *en lugar* (in-place) modificando directamente la estructura `&mut self`. La memoria auxiliar requerida es **$O(1)$**.

### Ejercicio 01: Combinación Lineal
* **Complejidad Temporal:** La función clona el primer vector y luego itera por cada uno de los $k-1$ vectores restantes. En cada iteración escala el vector temporal en tiempo $O(n)$ y lo suma al acumulador en tiempo $O(n)$. El tiempo total es **$O(k \cdot n)$**.
* **Complejidad Espacial:** Se crea el acumulador de tamaño $O(n)$ y se asigna un vector temporal de tamaño $O(n)$ por cada paso del bucle (que se libera de inmediato). Por ende, el consumo máximo de memoria simultánea es **$O(n)$**.

### Ejercicio 02: Interpolación Lineal
* **Complejidad Temporal:** La operación `u.clone() + (v - u) * t` requiere una resta de vectores ($O(n)$), una multiplicación escalar ($O(n)$) y una suma de vectores ($O(n)$). El tiempo total es **$O(n)$**.
* **Complejidad Espacial:** Genera estructuras intermedias para almacenar los resultados intermedios de las operaciones por valor. El consumo de memoria es **$O(n)$**.

### Ejercicio 03: Producto Escalar
* **Complejidad Temporal:** Realiza un bucle único que recorre simultáneamente ambos vectores (mediante `.zip()`), realizando $n$ multiplicaciones y sumas. Es **$O(n)$**.
* **Complejidad Espacial:** Solo almacena la variable acumuladora. El espacio auxiliar es **$O(1)$**.

### Ejercicio 04: Normas
* **Complejidad Temporal:** 
  * `norm_1` y `norm_inf` hacen una sola pasada lineal sobre las coordenadas ($O(n)$).
  * `norm` realiza un producto punto ($O(n)$) y luego una raíz cuadrada ($O(1)$).
  Todas son de tiempo **$O(n)$**.
* **Complejidad Espacial:** Espacio auxiliar constante **$O(1)$**.

### Ejercicio 05: Coseno
* **Complejidad Temporal:** Llama a `norm()` (dos veces) y a `dot()` (una vez). Cada llamada es $O(n)$, por lo que la complejidad temporal total es **$O(n)$**.
* **Complejidad Espacial:** Espacio auxiliar constante **$O(1)$**.

### Ejercicio 06: Producto Vectorial
* **Complejidad Temporal:** Restringido a dimensión 3. Ejecuta operaciones fijas (9 operaciones aritméticas). Es tiempo constante **$O(1)$**.
* **Complejidad Espacial:** Crea un único vector de 3 elementos. Espacio constante **$O(1)$**.

### Ejercicio 07: Multiplicación Matriz-Vector y Matriz-Matriz
* **Multiplicación Matriz-Vector:** Recorre las $m$ filas de la matriz y realiza un producto punto de tamaño $n$ para cada una. Tiempo: **$O(mn)$**. Espacio auxiliar: **$O(m)$** para el vector resultante.
* **Multiplicación Matriz-Matriz:** Tres bucles anidados de dimensiones $m$, $p$ y $n$. Tiempo: **$O(mnp)$**. Espacio auxiliar: **$O(mp)$** para alojar la matriz resultante $C$.

### Ejercicio 08: Traza
* **Complejidad Temporal:** Recorre únicamente los elementos de la diagonal `0..n`. Tiempo: **$O(n)$**.
* **Complejidad Espacial:** Espacio auxiliar constante **$O(1)$**.

### Ejercicio 09: Traspuesta
* **Complejidad Temporal:** Realiza dos bucles para copiar cada elemento $A_{ij}$ a la posición transpuesta $B_{ji}$. Tiempo: **$O(mn)$**.
* **Complejidad Espacial:** Aloja la nueva matriz transpuesta de dimensiones $n \times m$, ocupando **$O(mn)$** de memoria.

## Comparativa de Operaciones Elementales de Fila

Para resolver los algoritmos gaussianos (forma escalonada, determinante e inversa) de forma correcta, se aplican variaciones sobre las operaciones de fila:

| Operación | ¿Normalizas el pivote a 1? (Dividir fila entre pivote) | ¿Dónde haces ceros? | ¿Qué haces con los swaps? | Resultado |
| :--- | :--- | :--- | :--- | :--- |
| **`row_echelon`** | **SÍ** (fila = fila / pivote) | Arriba y abajo | Solo intercambias filas. | Matriz identidad o escalonada reducida (RREF). |
| **`determinant`** | **NO** (alteraría el determinante) | Solo abajo (triangular superior) | Inviertes el signo (`swapped = !swapped`). | $(\pm 1) \times$ Producto de la diagonal. |
| **`inverse`** | **SÍ** (en $A$ y en $I$) | Arriba y abajo (en $A$ y en $I$) | Intercambias filas en ambas matrices. | La matriz derecha se convierte en $A^{-1}$. |

---

### Ejercicio 10: Forma Escalonada por Filas (RREF)
* **Complejidad Temporal:** 
  * Bucle principal sobre las $n$ columnas.
  * Búsqueda de fila pivote e intercambio de filas: $O(m)$.
  * Normalización de la fila pivote: $O(n)$ divisiones.
  * Eliminación en las demás filas: $(m-1) \times O(n)$ multiplicaciones y restas.
  Esto resulta en una complejidad de $O(n \cdot (m + n + mn)) = O(mn^2)$. Si la matriz es cuadrada ($m=n$), equivale a **$O(n^3)$**.
* **Complejidad Espacial:** Clona la matriz original para devolver el resultado. Espacio total: **$O(mn)$** (o **$O(n^2)$** si es cuadrada).

### Ejercicio 11: Determinante
* **Complejidad Temporal:** Realiza la reducción Gaussiana (eliminación) para triangular la matriz. Bucle externo de $n$ pasos y eliminación de $O(n)$ operaciones por fila. Tiempo: **$O(n^3)$**.
* **Complejidad Espacial:** Clona la matriz para procesar la triangulación in-place. Espacio: **$O(n^2)$**.

### Ejercicio 12: Inversa
* **Complejidad Temporal:** Ejecuta el método de eliminación completa de Gauss-Jordan tanto sobre la matriz de trabajo como sobre la identidad. Tiempo: **$O(n^3)$**.
* **Complejidad Espacial:** Crea la matriz identidad inicial e itera sobre un clon. Espacio auxiliar: **$O(n^2)$**.

### Ejercicio 13: Rango
* **Complejidad Temporal:** Llama a `row_echelon()` ($O(n^3)$) y luego hace un barrido de las filas ($O(n^2)$) para contar las que son no nulas. Tiempo total: **$O(n^3)$**.
* **Complejidad Espacial:** Heredada de la llamada a `row_echelon()`, que clona la matriz. Espacio: **$O(n^2)$**.
