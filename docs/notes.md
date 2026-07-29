# Álgebra Lineal - Notas de Aprendizaje

## Capítulo 1: ¿Qué es un vector?
Este capítulo introduce el concepto fundamental del álgebra lineal: el **vector**, y explica cómo se visualiza y manipula tanto geométrica como numéricamente.

### 1. Las Tres Perspectivas de un Vector
Dependiendo del área de estudio, un vector se puede entender de tres maneras distintas:

*   **Perspectiva Física:** Un vector es una flecha apuntando en el espacio. Está definido por su **longitud (magnitud)** y su **dirección**. Puede moverse libremente por el espacio siempre que mantenga estas dos propiedades.
*   **Perspectiva de las Ciencias de la Computación (CS):** Un vector es una **lista ordenada de números**. Por ejemplo, si estamos analizando casas, podríamos representar una casa con un vector de dos dimensiones: `[precio, área]`. El número de elementos en la lista define la dimensión del vector.
*   **Perspectiva Matemática:** Es una generalización que unifica ambas visiones. Considera que un vector puede ser cualquier cosa (flechas, listas de números, funciones, etc.) siempre que cumpla con dos operaciones fundamentales: la **suma de vectores** y la **multiplicación por un escalar**.

---

### 2. Representación en el Plano Coordenado
En álgebra lineal, a diferencia de la física, casi siempre fijamos el origen de los vectores en el **punto inicial (0,0)** de un sistema de coordenadas.

*   Un vector en 2D se representa como un par de números ordenados colocados verticalmente en corchetes:
    $$\vec{v} = \begin{bmatrix} x \\ y \end{bmatrix}$$
*   El primer número ($x$) nos indica cuánto avanzar en el eje horizontal (hacia la derecha si es positivo, izquierda si es negativo).
*   El segundo número ($y$) nos indica cuánto avanzar en el eje vertical (hacia arriba si es positivo, abajo si es negativo).

---

### 3. Suma de Vectores
La suma de dos vectores se puede comprender desde dos enfoques:

#### A. Interpretación Geométrica (Método de la Cola a la Punta)
Para sumar geométricamente dos vectores $\vec{v}$ y $\vec{w}$:
1. Dibujamos el vector $\vec{v}$ con su cola en el origen.
2. Desplazamos el vector $\vec{w}$ de modo que su cola coincida con la punta de $\vec{v}$.
3. El vector resultante ($\vec{v} + \vec{w}$) es la flecha que va desde la cola del primer vector ($\vec{v}$) hasta la punta del segundo vector ($\vec{w}$).

> [!NOTE]
> **Intuición del desplazamiento:** Piensa en cada vector como una instrucción de movimiento en el espacio. Si realizas el movimiento indicado por $\vec{v}$ y a continuación el movimiento indicado por $\vec{w}$, el efecto neto de ambos trayectos es exactamente el vector suma $\vec{v} + \vec{w}$.

#### B. Interpretación Numérica
Numéricamente, sumamos elemento por elemento (coordenada a coordenada):
$$\begin{bmatrix} x_1 \\ y_1 \end{bmatrix} + \begin{bmatrix} x_2 \\ y_2 \end{bmatrix} = \begin{bmatrix} x_1 + x_2 \\ y_1 + y_2 \end{bmatrix}$$

---

### 4. Multiplicación por un Escalar
Consiste en multiplicar un vector por un número real.

#### A. Interpretación Geométrica (Escalado / *Scaling*)
Multiplicar un vector por un número cambia su longitud (magnitud) sin cambiar la línea de dirección sobre la que yace:
*   Si multiplicamos por $2$, duplicamos la longitud del vector.
*   Si multiplicamos por $0.5$, reducimos la longitud a la mitad.
*   Si multiplicamos por un número negativo (ej. $-1.8$), el vector cambia de sentido (apunta en la dirección opuesta) y se escala por el factor absoluto (en este caso, 1.8 veces más largo).

#### B. Interpretación Numérica
Numéricamente, multiplicamos cada componente del vector por el número:
$$c \cdot \begin{bmatrix} x \\ y \end{bmatrix} = \begin{bmatrix} c \cdot x \\ c \cdot y \end{bmatrix}$$

---

### 5. ¿Qué es un Escalar?
En álgebra lineal, la palabra **escalar** se refiere simplemente a un número real (como $2$, $-1$, o $0.5$).
*   **Origen del término:** Proviene de la acción de **escalar** (*scaling*), que es la función principal que realizan estos números al multiplicar a un vector geométrico (estirarlo, encogerlo o invertirlo en escala).

---

> [!IMPORTANT]
> Toda la estructura del álgebra lineal se sostiene sobre estas dos operaciones fundamentales: la **suma de vectores** y la **multiplicación por escalares**. A partir de ellas se construyen conceptos clave como las combinaciones lineales, el espacio generado (*span*), la independencia lineal y las transformaciones lineales.

---

## Capítulo 2: Combinaciones lineales, espacio generado y bases (*Linear combinations, span, and bases*)

Este capítulo profundiza en la relación entre las coordenadas numéricas y la geometría vectorial, introduciendo conceptos críticos para entender la estructura de los espacios vectoriales.

### 1. Una Nueva Perspectiva de las Coordenadas: Los Vectores Unitarios
En lugar de ver las coordenadas simplemente como "avanzar $x$ a la derecha e $y$ hacia arriba", podemos interpretarlas como **escalares que estiran o contraen vectores unitarios**.

*   **Vectores Unitarios (Base estándar):**
    *   $\hat{i}$ (leído "i-gorro" o "i-hat"): El vector unitario apuntando hacia la derecha en el eje $X$. $\hat{i} = \begin{bmatrix} 1 \\ 0 \end{bmatrix}$.
    *   $\hat{j}$ (leído "j-gorro" o "j-hat"): El vector unitario apuntando hacia arriba en el eje $Y$. $\hat{j} = \begin{bmatrix} 0 \\ 1 \end{bmatrix}$.
*   **Reinterpretación:** Cualquier vector $\begin{bmatrix} x \\ y \end{bmatrix}$ es en realidad la **suma de dos vectores escalados**:
    $$\vec{v} = x\hat{i} + y\hat{j}$$
    Aquí, $x$ e $y$ son los escalares que dictan cuánto escalar a $\hat{i}$ y a $\hat{j}$.
*   A estos vectores especiales ($\hat{i}$ y $\hat{j}$) se les conoce como la **base** de nuestro sistema de coordenadas estándar.

---

### 2. Combinaciones Lineales
Cuando escalamos dos o más vectores y sumamos los resultados, la operación se conoce como una **combinación lineal**:
$$a\vec{v} + b\vec{w}$$
Donde $a$ y $b$ son escalares.

#### ¿De dónde viene el término "lineal"?
*   Si fijas uno de los escalares (por ejemplo, $b=1$) y permites que el otro ($a$) varíe libremente, la punta del vector resultante dibujará una **línea recta** en el plano.
*   Si dejas que ambos escalares se muevan libremente, para la mayoría de los pares de vectores en 2D, podrás alcanzar **cualquier punto del plano bidimensional**.
*   **Excepciones:**
    *   Si los dos vectores originales están alineados (tienen la misma dirección o son colineales), la punta del vector resultante estará limitada a la **línea recta** que pasa por el origen.
    *   Si ambos vectores son el vector cero $\vec{0}$, quedarás atrapado en el **origen**.

---

### 3. El Espacio Generado (*Span*)
El **espacio generado (o *span*)** de un conjunto de vectores es el **conjunto de todos los vectores resultantes** que se pueden alcanzar mediante combinaciones lineales de ese grupo de vectores.

*   **En 2D:**
    *   Para la mayoría de pares de vectores, su *span* es **todo el plano 2D**.
    *   Si están alineados, su *span* es una **línea recta**.
    *   Si ambos son cero, su *span* es solo el **origen**.

#### Vectores frente a Puntos
> [!TIP]
> **Consejo visual:** Si piensas en un vector de forma aislada, es útil imaginarlo como una **flecha**. Sin embargo, si vas a visualizar una colección enorme (como un *span*), dibujar miles de flechas causa saturación visual. En su lugar, representa cada vector solo por su **punto final (punta)**. Así, el *span* de la mayoría de pares de vectores en 2D se visualiza como una hoja o plano infinito de puntos.

---

### 4. El Espacio Generado en 3D
Si tenemos tres vectores en un espacio tridimensional, su combinación lineal se expresa como:
$$a\vec{v} + b\vec{w} + c\vec{u}$$

El espacio generado por tres vectores depende de cómo interactúan entre sí:
1.  **Caso de alineación (Sin valor agregado):** Si sumas un tercer vector $\vec{u}$ que ya se encuentra en el plano generado por $\vec{v}$ y $\vec{w}$, el *span* **no cambia** (sigues atrapado en la misma hoja bidimensional). El tercer vector no te da acceso a nuevas dimensiones.
2.  **Caso general (Desbloqueo tridimensional):** Si el tercer vector apunta en una dirección diferente (fuera del plano generado por los otros dos), entonces desbloqueas todo el **espacio tridimensional (3D)**. A medida que cambias el escalar $c$, estás moviendo el plano generado por los dos primeros vectores a lo largo de la dirección del tercero, barriendo todo el volumen 3D.

---

### 5. Dependencia e Independencia Lineal

*   **Linealmente Dependientes:** Se da cuando tienes un conjunto de vectores donde al menos uno de ellos es redundante, es decir, ya está en el espacio generado por los demás (puedes quitarlo sin alterar el *span* del conjunto).
    *   *Ejemplo:* $\vec{u} = a\vec{v} + b\vec{w}$ (el vector $\vec{u}$ depende de $\vec{v}$ y $\vec{w}$).
*   **Linealmente Independientes:** Se da cuando cada vector del conjunto aporta una nueva dimensión al espacio generado (ningún vector se puede expresar como una combinación lineal de los demás).
    *   *Ejemplo:* $\vec{w} \neq a\vec{v}$ para todo escalar $a$ (ninguno es colineal con el otro).

---

### 6. Definición Técnica de Base
Una **base** de un espacio vectorial es un conjunto de vectores que cumple dos propiedades fundamentales:
1.  Son **linealmente independientes**.
2.  Su espacio generado (*span*) cubre **todo ese espacio vectorial**.

> [!IMPORTANT]
> Los vectores de una base actúan como los "bloques de construcción" mínimos y eficientes necesarios para describir de forma única cualquier vector de ese espacio sin redundancias.

---

## Capítulo 3: Transformaciones lineales y matrices (*Linear transformations and matrices*)

Este capítulo explora qué son las transformaciones lineales, cómo visualizarlas a través del movimiento del espacio, y cómo representarlas numéricamente usando matrices.

### 1. ¿Qué es una Transformación?
En álgebra lineal, el término **transformación** es fundamentalmente un sinónimo de **función**: toma un vector de entrada y nos devuelve un vector de salida.
*   **¿Por qué usar "transformación" en vez de "función"?**
    *   Sugiere una forma visual de conceptualizar el proceso. En lugar de ver las entradas y salidas de forma estática, imaginamos que el vector de entrada **se mueve** o se desplaza hacia su correspondiente vector de salida.
    *   Para visualizar la transformación por completo, podemos imaginar cómo **todo el espacio (la cuadrícula de coordenadas)** se estira, se comprime o rota simultáneamente a medida que los vectores de entrada se transforman en sus salidas correspondientes.

---

### 2. ¿Cuándo es "Lineal" una Transformación?
Geométricamente, una transformación en el plano es **lineal** si cumple dos propiedades muy estrictas:
1.  **Las líneas deben seguir siendo líneas rectas** (las líneas de la cuadrícula no pueden curvarse ni doblarse).
2.  **El origen debe permanecer fijo** en el mismo lugar ($(0,0)$ no se mueve).

> [!NOTE]
> En resumen, una transformación es lineal si las líneas de la cuadrícula original permanecen **paralelas y espaciadas uniformemente** después del movimiento.

---

### 3. Descripción Numérica de una Transformación Lineal
A primera vista, describir hacia dónde se mueve cada uno de los infinitos vectores del plano parece una tarea imposible. Sin embargo, gracias a la linealidad, existe una propiedad asombrosa:

> **Propiedad fundamental:**
> Para conocer y describir una transformación lineal por completo, **solo necesitas registrar a dónde van a parar los vectores de la base estándar ($\hat{i}$ y $\hat{j}$)**. La posición final de cualquier otro vector del plano se puede deducir directamente de esto.

#### Ejemplo Paso a Paso:
Supongamos que tenemos un vector $\vec{v} = \begin{bmatrix} -1 \\ 2 \end{bmatrix}$. Esto significa que:
$$\vec{v} = -1\hat{i} + 2\hat{j}$$

Si aplicamos una transformación lineal al plano, la relación lineal se conserva perfectamente. Por lo tanto, el vector transformado resultante ($\vec{v}_{\text{transformado}}$) será:
$$\vec{v}_{\text{transformado}} = -1(\hat{i}_{\text{transformado}}) + 2(\hat{j}_{\text{transformado}})$$

Si sabemos que tras la transformación:
*   $\hat{i}$ aterriza en $\begin{bmatrix} 1 \\ -2 \end{bmatrix}$
*   $\hat{j}$ aterriza en $\begin{bmatrix} 3 \\ 0 \end{bmatrix}$

Entonces, podemos calcular con absoluta certeza dónde aterriza el vector $\vec{v}$:
$$\vec{v}_{\text{transformado}} = -1 \begin{bmatrix} 1 \\ -2 \end{bmatrix} + 2 \begin{bmatrix} 3 \\ 0 \end{bmatrix} = \begin{bmatrix} -1 \\ 2 \end{bmatrix} + \begin{bmatrix} 6 \\ 0 \end{bmatrix} = \begin{bmatrix} 5 \\ 2 \end{bmatrix}$$

---

### 4. La Conexión con las Matrices
Toda la información necesaria para definir una transformación lineal en 2D se puede empaquetar de forma compacta en una **matriz de $2 \times 2$**, donde cada columna representa el vector destino de nuestros vectores de base:

$$\text{Matriz de Transformación} = \begin{bmatrix} | & | \\ \hat{i}_{\text{transformado}} & \hat{j}_{\text{transformado}} \\ | & | \end{bmatrix}$$

Usando los datos del ejemplo anterior:
$$\text{Matriz} = \begin{bmatrix} 1 & 3 \\ -2 & 0 \end{bmatrix}$$

De esta manera, la multiplicación de una matriz por un vector no es más que una abreviación para calcular la combinación lineal de los vectores columnas de la matriz, escalados por las componentes del vector:

$$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} = x \begin{bmatrix} a \\ c \end{bmatrix} + y \begin{bmatrix} b \\ d \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}$$

> [!IMPORTANT]
> Siempre que veas una matriz, piensa geométricamente: **sus columnas son los vectores donde aterrizan los vectores unitarios de la base ($\hat{i}$, $\hat{j}$, etc.) tras la transformación.**

---

## Capítulo 4: Multiplicación de matrices como composición (*Matrix multiplication as composition*)

Este capítulo explica cómo combinar múltiples transformaciones lineales en una sola operación matemática mediante la multiplicación de matrices, y por qué esta operación se comporta de la manera en que lo hace.

### 1. Composición de Transformaciones
A menudo queremos aplicar una transformación lineal al espacio y, inmediatamente después, aplicar otra.
*   **Ejemplo:** Una **rotación** (girar el espacio alrededor del origen) seguida de un **cizallamiento** o **esfuerzo cortante** (*shear*).
    *   *¿Qué es un cizallamiento (shear)?* Es una transformación que desplaza los puntos en una dirección paralela a un eje fijo por una distancia proporcional a su distancia perpendicular a dicho eje. Visualmente, es como empujar lateralmente la parte superior de un mazo de cartas o un marco rectangular, convirtiendo los cuadrados en paralelogramos mientras la base se queda fija.

Si aplicamos primero una rotación a un vector y luego un cizallamiento al resultado, el proceso completo es la **composición** de ambas transformaciones.
*   Geométricamente, esta composición sigue siendo una transformación lineal (las líneas siguen siendo rectas y paralelas, y el origen sigue fijo). Por tanto, debe poder expresarse mediante una **única matriz**.

---

### 2. Lectura de Derecha a Izquierda (Notación de Composición)
Numéricamente, si queremos aplicar primero una matriz de rotación ($R$) y luego una matriz de cizallamiento ($S$) a un vector $\vec{v}$:
1.  Aplicamos la rotación: $R\vec{v}$
2.  Aplicamos el cizallamiento al resultado: $S(R\vec{v})$

Por la propiedad asociativa, esto se escribe como:
$$(S \cdot R)\vec{v}$$

> [!IMPORTANT]
> **Orden de lectura:** Al igual que en la notación de funciones en cálculo $f(g(x))$, donde se aplica primero la función más interna ($g$) y luego la externa ($f$), en el álgebra lineal las matrices se leen de **derecha a izquierda**:
> $$\text{Matriz Final} = \text{Matriz}_2(\text{Segunda Acción}) \cdot \text{Matriz}_1(\text{Primera Acción})$$

---

### 3. Deducir la Multiplicación de Matrices Geométricamente
Para encontrar el producto de dos matrices $M_2 \cdot M_1$, no necesitamos memorizar fórmulas complejas; solo debemos rastrear a dónde van a parar los vectores base $\hat{i}$ y $\hat{j}$ tras aplicar ambas transformaciones secuencialmente.

Definamos las dos matrices:
*   **Primera Transformación ($M_1$):** $\begin{bmatrix} e & f \\ g & h \end{bmatrix}$ (donde $\hat{i} \to \begin{bmatrix} e \\ g \end{bmatrix}$ y $\hat{j} \to \begin{bmatrix} f \\ h \end{bmatrix}$)
*   **Segunda Transformación ($M_2$):** $\begin{bmatrix} a & b \\ c & d \end{bmatrix}$

#### Rastreando a $\hat{i}$ (Columna 1 del resultado):
1.  Tras la primera transformación ($M_1$), $\hat{i}$ aterriza en el vector $\begin{bmatrix} e \\ g \end{bmatrix}$.
2.  Al aplicar la segunda transformación ($M_2$) sobre este nuevo vector, obtenemos:
    $$M_2 \begin{bmatrix} e \\ g \end{bmatrix} = e \begin{bmatrix} a \\ c \end{bmatrix} + g \begin{bmatrix} b \\ d \end{bmatrix} = \begin{bmatrix} ae + bg \\ ce + dg \end{bmatrix}$$
    Esta combinación lineal es la **primera columna** de nuestra matriz compuesta.

#### Rastreando a $\hat{j}$ (Columna 2 del resultado):
1.  Tras la primera transformación ($M_1$), $\hat{j}$ aterriza en el vector $\begin{bmatrix} f \\ h \end{bmatrix}$.
2.  Al aplicar la segunda transformación ($M_2$) sobre este vector, obtenemos:
    $$M_2 \begin{bmatrix} f \\ h \end{bmatrix} = f \begin{bmatrix} a \\ c \end{bmatrix} + h \begin{bmatrix} b \\ d \end{bmatrix} = \begin{bmatrix} af + bh \\ cf + dh \end{bmatrix}$$
    Esta combinación lineal es la **segunda columna** de nuestra matriz compuesta.

#### Fórmula General del Producto de Matrices $2 \times 2$:
$$\begin{bmatrix} a & b \\ c & d \end{bmatrix} \begin{bmatrix} e & f \\ g & h \end{bmatrix} = \begin{bmatrix} ae + bg & af + bh \\ ce + dg & cf + dh \end{bmatrix}$$

---

### 4. El Orden Importa (No Conmutatividad)
Una característica clave de la multiplicación de matrices es que **no es conmutativa**:
$$M_2 \cdot M_1 \neq M_1 \cdot M_2$$

*   **Explicación Geométrica:** El orden en que aplicas las transformaciones altera por completo el resultado final.
    *   Si primero rotas el plano $90^\circ$ y luego realizas un cizallamiento horizontal, obtendrás un resultado muy diferente a si realizas primero el cizallamiento horizontal y después rotas todo el plano $90^\circ$.



