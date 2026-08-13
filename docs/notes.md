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
---

## Capítulo 5: Transformaciones lineales en el espacio tridimensional (*Three-dimensional linear transformations*)

Este capítulo extiende los conceptos de transformaciones lineales y matrices al espacio tridimensional (3D), mostrando cómo la misma intuición geométrica de rastrear vectores base se mantiene al añadir una dimensión extra.

### 1. La Base Estándar y Coordenadas en 3D
En el espacio tridimensional, un vector se define usando tres coordenadas: $x$, $y$ y $z$.
*   **Vectores Unitarios (Base Estándar):**
    *   $\hat{i}$ (eje $X$): $\begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix}$
    *   $\hat{j}$ (eje $Y$): $\begin{bmatrix} 0 \\ 1 \\ 0 \end{bmatrix}$
    *   $\hat{k}$ (eje $Z$): $\begin{bmatrix} 0 \\ 0 \\ 1 \end{bmatrix}$

Cualquier vector $\vec{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix}$ puede interpretarse como una instrucción de cómo escalar y sumar estos tres vectores base:
$$\vec{v} = x\hat{i} + y\hat{j} + z\hat{k}$$

#### Intuición Visual de la Composición de un Vector:
```
       Y (j-hat)
       ^
       |   . v = x*i + y*j + z*k
       |  /
       | / 
       |/________> X (i-hat)
      /
     /
    v
   Z (k-hat)
```

---

### 2. Transformaciones Lineales en 3D
Al igual que en 2D, una transformación en 3D es **lineal** si cumple que:
1.  El origen permanece fijo en $(0,0,0)$.
2.  Todas las líneas de la cuadrícula tridimensional siguen siendo líneas rectas paralelas y espaciadas de forma uniforme.

Para describir completamente hacia dónde se desplaza todo el espacio 3D, **solo necesitamos rastrear dónde aterrizan los tres vectores base estándar**: $\hat{i}$, $\hat{j}$ y $\hat{k}$ tras la transformación.

---

### 3. Matrices de $3 \times 3$
Empaquetamos las nuevas posiciones de los tres vectores base como las columnas de una matriz de $3 \times 3$:

$$\text{Matriz de Transformación 3D} = \begin{bmatrix} | & | & | \\ \hat{i}_{\text{transformado}} & \hat{j}_{\text{transformado}} & \hat{k}_{\text{transformado}} \\ | & | & | \end{bmatrix}$$

---

### 4. Ejemplo Práctico: Rotación de $90^\circ$ alrededor del eje Y
Consideremos una transformación que rota el espacio tridimensional $90^\circ$ alrededor del eje $Y$.

*   **¿Qué ocurre con cada vector base?**
    *   $\hat{j}$ (el eje de rotación) **no se mueve**:
        $$\hat{j}_{\text{transformado}} = \begin{bmatrix} 0 \\ 1 \\ 0 \end{bmatrix}$$
    *   $\hat{i}$ (eje $X$ positivo) y $\hat{k}$ (eje $Z$ positivo) rotan sobre el plano $XZ$:
        *   **Si la rotación es en sentido horario** (visto desde el extremo positivo del eje Y):
            $\hat{i}$ va al eje $Z$ positivo: $\hat{i}_{\text{transformado}} = \begin{bmatrix} 0 \\ 0 \\ 1 \end{bmatrix}$
            $\hat{k}$ va al eje $X$ negativo: $\hat{k}_{\text{transformado}} = \begin{bmatrix} -1 \\ 0 \\ 0 \end{bmatrix}$
            La matriz de esta rotación es:
            $$R_{\text{horaria}} = \begin{bmatrix} 0 & 0 & -1 \\ 0 & 1 & 0 \\ 1 & 0 & 0 \end{bmatrix}$$
        *   **Si la rotación es en sentido antihorario** (visto desde el extremo positivo del eje Y, regla de la mano derecha):
            $\hat{i}$ va al eje $Z$ negativo: $\hat{i}_{\text{transformado}} = \begin{bmatrix} 0 \\ 0 \\ -1 \end{bmatrix}$
            $\hat{k}$ va al eje $X$ positivo: $\hat{k}_{\text{transformado}} = \begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix}$
            La matriz de esta rotación es:
            $$R_{\text{antihoraria}} = \begin{bmatrix} 0 & 0 & 1 \\ 0 & 1 & 0 \\ -1 & 0 & 0 \end{bmatrix}$$

> [!NOTE]
> Identificar a dónde va a parar cada vector base de manera independiente simplifica enormemente la construcción de matrices tridimensionales que de otro modo parecerían extremadamente complejas de deducir.

---

### 5. Multiplicación de Vector por Matriz en 3D
Para ver dónde cae un vector cualquiera $\vec{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix}$ después de la transformación, usamos el mismo principio de linealidad que en 2D. 

El proceso de escalar y sumar funciona **tanto antes como después de la transformación**:
$$\vec{v}_{\text{transformado}} = x(\hat{i}_{\text{transformado}}) + y(\hat{j}_{\text{transformado}}) + z(\hat{k}_{\text{transformado}})$$

#### Expresión Matemática:
$$\begin{bmatrix} a & b & c \\ d & e & f \\ g & h & i \end{bmatrix} \begin{bmatrix} x \\ y \\ z \end{bmatrix} = x \begin{bmatrix} a \\ d \\ g \end{bmatrix} + y \begin{bmatrix} b \\ e \\ h \end{bmatrix} + z \begin{bmatrix} c \\ f \\ i \end{bmatrix} = \begin{bmatrix} ax + by + cz \\ dx + ey + fz \\ gx + hy + iz \end{bmatrix}$$

---

### 6. Composición de Transformaciones en 3D (Multiplicación de Matrices)
La multiplicación de dos matrices de $3 \times 3$ representa la composición de dos transformaciones lineales en el espacio tridimensional.
*   **Lectura de Derecha a Izquierda:** Al igual que en 2D, para calcular el efecto de aplicar la transformación $A$ y luego la transformación $B$, multiplicamos las matrices como $B \cdot A$. Esto significa aplicar primero la transformación de la derecha ($A$) y luego la de la izquierda ($B$).

#### Relevancia en Gráficos por Computadora y Robótica
> [!TIP]
> **Importancia en Ingeniería:**
> En campos como el modelado 3D, desarrollo de videojuegos y robótica (cinemática de brazos robóticos, drones, etc.), describir rotaciones tridimensionales complejas directamente puede ser sumamente difícil y propenso a errores. 
> 
> Es mucho más sencillo imaginar y modelar estas transformaciones dividiéndolas en una **composición de rotaciones más simples y distintas** (por ejemplo, rotar primero alrededor del eje $X$, luego alrededor de $Y$, y finalmente alrededor de $Z$). Multiplicar estas matrices simples nos da la matriz final de orientación de manera automática y limpia.

---

## Capítulo 6: El Determinante (*The Determinant*)

Este capítulo introduce el concepto de **determinante**, una medida fundamental que describe cómo una transformación lineal altera las áreas (en 2D) o los volúmenes (en 3D) del espacio.

### 1. El Determinante como Factor de Escala
Para entender el impacto de una transformación lineal, es sumamente útil medir cuánto estira o comprime el espacio. Esto equivale a medir **el factor por el cual el área de una región cualquiera se incrementa o disminuye**.

#### Ejemplos en 2D:
1.  **Escalado Puro:**
    $$M_1 = \begin{bmatrix} 3 & 0 \\ 0 & 2 \end{bmatrix}$$
    Esta matriz estira el vector base $\hat{i}$ por un factor de $3$ en el eje $X$, y el vector $\hat{j}$ por un factor de $2$ en el eje $Y$. El cuadrado unitario original de $1 \times 1$ (área 1) se convierte en un rectángulo de $3 \times 2$ (área 6). Por lo tanto, el área se ha escalado por un factor de $6$. El determinante de esta transformación es $6$.
    
2.  **Cizallamiento (Shear):**
    $$M_2 = \begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix}$$
    Aunque deforma la cuadrícula convirtiendo los cuadrados en paralelogramos, la base de los paralelogramos sigue siendo de longitud $1$ y la altura sigue siendo $1$. Esto significa que el área de cualquier región se mantiene intacta. El determinante de esta transformación es $1$.

> [!NOTE]
> **Generalización del factor de escala:**
> Debido a que las transformaciones lineales mantienen las líneas de la cuadrícula paralelas y uniformemente espaciadas, **cualquier figura** (notablemente las que no son cuadradas) experimenta exactamente el mismo factor de escala en su área. Las figuras irregulares se pueden aproximar mediante una cuadrícula de cuadraditos infinitamente pequeños; dado que el área de cada cuadradito se multiplica por el determinante, el área total de la figura también lo hace.

---

### 2. Significado de los Valores del Determinante
El determinante es un número real que nos da información crucial sobre la transformación:
*   **$\det(T) = 3$**: El área de cualquier región se triplica.
*   **$\det(T) = 0.5$**: El área se reduce a la mitad.
*   **$\det(T) = 0$**: La transformación comprime todo el plano bidimensional en una línea o en un solo punto. La nueva área de cualquier figura es cero.
    *   *Ejemplo de compresión a una línea:* $M = \begin{bmatrix} 1 & 2 \\ 1 & 2 \end{bmatrix}$. Las columnas son colineales, por lo que todo el plano colapsa a la recta $y = x$.
    *   *Ejemplo de compresión a un punto:* $M = \begin{bmatrix} 0 & 0 \\ 0 & 0 \end{bmatrix}$. Todo el espacio colapsa al origen.

> [!IMPORTANT]
> Verificar si el determinante de una matriz es $0$ es sumamente importante, ya que nos indica si la transformación **comprime el espacio a una dimensión menor**, haciéndole perder información (es decir, la transformación no es invertible).

---

### 3. Determinantes Negativos y Orientación
En álgebra lineal, el determinante puede tomar valores negativos. Dado que un área no puede ser físicamente negativa, el signo negativo representa un cambio en la **orientación** del espacio.

*   **Flipped Space (Inversión):** En 2D, un determinante negativo significa que la transformación "le dio la vuelta a la hoja" sobre la que están dibujados los vectores.
*   **Rastreo de la Orientación:**
    *   En el plano estándar, si recorremos de $\hat{i}$ a $\hat{j}$, nos movemos en sentido antihorario.
    *   Si tras la transformación, el camino para ir de $\hat{i}_{\text{transformado}}$ a $\hat{j}_{\text{transformado}}$ pasa a ser en **sentido horario**, la orientación se ha invertido y el determinante es negativo.
    *   *Ejemplo:*
        $$\det\left(\begin{bmatrix} 2 & 1 \\ -1 & -3 \end{bmatrix}\right) = (2)(-3) - (1)(-1) = -6 + 1 = -5$$
        El signo negativo indica que la orientación se invirtió, pero el valor absoluto ($|{-5}| = 5$) nos sigue indicando que el área de cualquier figura se escala por un factor de $5$.

---

### 4. El Determinante en el Espacio Tridimensional (3D)
En 3D, el determinante mide cómo cambian los **volúmenes** bajo la transformación.

*   **El Cubo Unitario:** En lugar de rastrear un cuadrado de área 1, rastreamos el volumen del cubo unitario de $1 \times 1 \times 1$ cuyas aristas están dadas por los vectores base $\hat{i}$, $\hat{j}$ y $\hat{k}$.
*   **Paralelepípedo:** Tras la transformación, este cubo unitario se deforma en un paralelepípedo tridimensional. El determinante de la matriz $3 \times 3$ es exactamente el **volumen** (con signo) de este paralelepípedo.
    *   *Ejemplo:*
        $$\det\left(\begin{bmatrix} 1 & 0 & 0.5 \\ 0.5 & 1 & 0 \\ 1 & 0 & 1 \end{bmatrix}\right)$$

#### Casos Especiales en 3D:
*   **$\det(T) = 0$:** Significa que el cubo colapsó por completo, comprimiendo todo el espacio 3D en un plano bidimensional, una línea unidimensional o un punto en el origen. Esto nos indica que los tres vectores columnas de la matriz son **linealmente dependientes**.
*   **$\det(T) < 0$:** Significa que la orientación del espacio 3D se ha invertido.

#### La Regla de la Mano Derecha para Orientación 3D:
Para verificar la orientación en 3D:
1.  Alinea tu mano derecha de modo que el dedo **índice** apunte hacia $\hat{i}$, el **medio** hacia $\hat{j}$ y el **pulgar** hacia $\hat{k}$.
2.  Si tras la transformación, todavía puedes representar la orientación relativa de los vectores transformados ($\hat{i}_{\text{transformado}}$, $\hat{j}_{\text{transformado}}$, $\hat{k}_{\text{transformado}}$) con tu **mano derecha**, la orientación se conserva y el determinante es **positivo**.
3.  Si necesitas usar la **mano izquierda** para que coincidan los dedos, la orientación se ha invertido (reflejado) y el determinante es **negativo**.

---

### 5. Cálculo del Determinante

#### En Dos Dimensiones (2D)
Para una matriz de $2 \times 2$:
$$\det\left(\begin{bmatrix} a & b \\ c & d \end{bmatrix}\right) = ad - bc$$

##### Justificación Geométrica:
*   Si $b = 0$ y $c = 0$, el vector $\hat{i}$ se estira por $a$ en el eje $X$ y $\hat{j}$ por $d$ en el eje $Y$. El área del rectángulo resultante es $a \cdot d$.
*   Si solo una de las esquinas es distinta de cero (por ejemplo, $b \neq 0$ pero $c = 0$), obtenemos un paralelogramo inclinado. Como la base sigue siendo de longitud $a$ y la altura perpendicular sigue siendo $d$, el área se mantiene como $a \cdot d$.
*   Si tanto $b$ como $c$ son distintos de cero, el término $-bc$ actúa como un factor de corrección por el estiramiento diagonal del paralelogramo. Geométricamente:
    *   El área total del rectángulo contenedor es $(a+b)(c+d) = ac + ad + bc + bd$.
    *   Restando las áreas exteriores al paralelogramo (dos rectángulos de área $bc$ y cuatro triángulos cuya suma de áreas es $ac + bd$), obtenemos la fórmula exacta:
        $$\text{Área} = (a+b)(c+d) - ac - bd - 2bc = ad - bc$$

#### En Tres Dimensiones (3D)
Para una matriz de $3 \times 3$, el determinante se calcula mediante la expansión por cofactores a lo largo de la primera fila:
$$\det\left(\begin{bmatrix} a & b & c \\ d & e & f \\ g & h & i \end{bmatrix}\right) = a \det\left(\begin{bmatrix} e & f \\ h & i \end{bmatrix}\right) - b \det\left(\begin{bmatrix} d & f \\ g & i \end{bmatrix}\right) + c \det\left(\begin{bmatrix} d & e \\ g & h \end{bmatrix}\right)$$

Donde cada término multiplica un elemento de la primera fila por el determinante de la matriz de $2 \times 2$ restante al eliminar su respectiva fila y columna. El signo del segundo término se alterna ($-$ $b$).

---

### 6. Propiedad Multiplicativa: $\det(M_1 \cdot M_2) = \det(M_1) \cdot \det(M_2)$
*   **Explicación Geométrica:**
    Aplicar la composición de transformaciones $M_1 \cdot M_2$ sobre el espacio equivale a aplicar primero la transformación $M_2$ (que escala todas las áreas o volúmenes por un factor de $\det(M_2)$) y después la transformación $M_1$ (que escala el espacio ya transformado por un factor de $\det(M_1)$). Por lo tanto, el factor de escala total acumulado del área o volumen es el producto de ambos determinantes: $\det(M_1) \cdot \det(M_2)$.

---

## Capítulo 7: Matrices inversas, espacio columna y espacio nulo (*Inverse matrices, column space, and null space*)

Este capítulo explora cómo resolver sistemas de ecuaciones lineales desde una perspectiva geométrica, introduciendo los conceptos clave de la matriz inversa, el rango, el espacio columna y el espacio nulo (o núcleo).

### 1. Sistemas de Ecuaciones Lineales
El álgebra lineal nos permite resolver ciertos sistemas de ecuaciones lineales, es decir, una lista de variables ($x, y, z$) que no conocemos y una lista de ecuaciones que las relacionan.

En una ecuación lineal, lo único que le ocurre a cada variable es que es multiplicada (escalada) por una constante, y estas variables escaladas se suman entre sí. No hay exponentes (como $x^2$), funciones no lineales (como $\sin(x)$), ni multiplicaciones de variables entre sí (como $x \cdot y$).

#### A. Organización del Sistema
Es recomendable organizar el sistema de la siguiente manera:
1. Dejar todas las variables a la izquierda y las constantes a la derecha del signo igual ($=$).
2. Alinear verticalmente las mismas variables. Si alguna variable no aparece en una ecuación, se le asigna un coeficiente de $0$.

Por ejemplo, el sistema:
$$2x + 5y + 3z = -3$$
$$4x + 8z = 0$$
$$x + 3y = 2$$

Se puede reescribir con coeficientes explícitos:
$$2x + 5y + 3z = -3$$
$$4x + 0y + 8z = 0$$
$$1x + 3y + 0z = 2$$

#### B. La Ecuación Vectorial ($A\vec{x} = \vec{v}$)
Este sistema se puede expresar elegantemente como el producto de una matriz por un vector:
$$\begin{bmatrix} 2 & 5 & 3 \\ 4 & 0 & 8 \\ 1 & 3 & 0 \end{bmatrix} \begin{bmatrix} x \\ y \\ z \end{bmatrix} = \begin{bmatrix} -3 \\ 0 \\ 2 \end{bmatrix}$$

De manera compacta:
$$A\vec{x} = \vec{v}$$

*   $A$: Es la matriz que representa una transformación lineal del espacio.
*   $\vec{x}$: Es el vector incógnita que queremos encontrar.
*   $\vec{v}$: Es el vector destino resultante tras la transformación.

> [!NOTE]
> **Interpretación Geométrica:**
> Resolver $A\vec{x} = \vec{v}$ significa buscar un vector $\vec{x}$ en el espacio de entrada tal que, tras aplicar la transformación lineal $A$, su vector resultante aterrice exactamente en el vector destino $\vec{v}$.

---

### 2. El Caso Invertible (Determinante $\neq 0$)
Para empezar de forma sencilla, supongamos un sistema de 2 ecuaciones con 2 incógnitas, de modo que $A$ es una matriz de $2 \times 2$, y tanto $\vec{x}$ como $\vec{v}$ son vectores bidimensionales.

El caso más común y probable es que el determinante de la matriz $A$ sea diferente de cero ($\det(A) \neq 0$). Esto significa geométricamente que la transformación no comprime el espacio a una dimensión menor (el plano 2D sigue siendo un plano 2D, con un área mayor a cero).

#### A. La Matriz Inversa ($A^{-1}$)
Si el espacio no se ha colapsado, existe una transformación que deshace o revierte el efecto de $A$. Esta transformación inversa se representa por la **matriz inversa**, denotada como $A^{-1}$.

*   **Propiedad fundamental:**
    Si aplicamos la transformación $A$ y luego aplicamos $A^{-1}$, el efecto neto es no hacer nada. Esto equivale a la **matriz identidad** ($I$), que deja todos los vectores intactos:
    $$A^{-1} \cdot A = I = \begin{bmatrix} 1 & 0 \\ 0 & 1 \end{bmatrix}$$

#### B. Resolviendo el Sistema
Geométricamente, para encontrar qué vector $\vec{x}$ aterriza en $\vec{v}$ bajo la transformación $A$, podemos simplemente "jugar la película al revés" aplicando la transformación inversa $A^{-1}$ sobre el vector destino $\vec{v}$:
$$\vec{x} = A^{-1}\vec{v}$$

Numéricamente, si multiplicamos ambos lados de la ecuación original por la izquierda por $A^{-1}$:
$$A^{-1}(A\vec{x}) = A^{-1}\vec{v}$$
$$(A^{-1}A)\vec{x} = A^{-1}\vec{v}$$
$$I\vec{x} = A^{-1}\vec{v}$$
$$\vec{x} = A^{-1}\vec{v}$$

Dado que el espacio no se colapsó, esta solución $\vec{x}$ es **única**. Hay un único vector que aterriza en $\vec{v}$.

---

### 3. El Caso No Invertible (Determinante $= 0$)
¿Qué sucede si $\det(A) = 0$? 
Geométricamente, esto significa que la transformación lineal **colapsa el espacio** en una dimensión menor (por ejemplo, el plano 2D se aplasta a una línea recta, o a un único punto en el origen).

En este escenario:
*   **No existe una matriz inversa $A^{-1}$:** No se puede "des-colapsar" una línea para volver a formar un plano. Requeriría transformar una única entrada (un punto en la línea) en infinitas salidas (los puntos que colapsaron allí), lo cual deja de ser una función o transformación lineal válida. No puedes recuperar la información perdida.

#### ¿Tiene solución el sistema $A\vec{x} = \vec{v}$ cuando $\det(A) = 0$?
Depende de la posición de $\vec{v}$:
1.  **Sin Solución:** Si el vector destino $\vec{v}$ se encuentra fuera de la línea (o plano) en la que se colapsó el espacio, es imposible encontrar un vector $\vec{x}$ que aterrice en él. No hay combinaciones lineales de las columnas de $A$ que alcancen a $\vec{v}$.
2.  **Infinitas Soluciones:** Si el vector destino $\vec{v}$ yace exactamente sobre la línea (o plano) en la que se colapsó el espacio, entonces habrá infinitas soluciones. Una línea completa de vectores de entrada se aplastará de modo que todos aterricen sobre $\vec{v}$.

---

### 4. Espacio Columna (*Column Space*) y Rango (*Rank*)

#### A. Rango (*Rank*)
El **rango** de una transformación lineal (o de su matriz asociada) es el **número de dimensiones** del espacio resultante después de aplicar la transformación.
*   **En 2D:**
    *   Si el espacio resultante es un plano completo (no colapsó), el rango es **2**.
    *   Si el plano se comprimió en una línea recta, el rango es **1**.
    *   Si el plano colapsó por completo en el origen, el rango es **0**.
*   **Matriz de Rango Completo (*Full Rank*):**
    Ocurre cuando el rango es el máximo posible para el tamaño de la matriz (por ejemplo, rango 2 para una matriz de $2 \times 2$, o rango 3 para una matriz de $3 \times 3$). Esto equivale a decir que el determinante es distinto de cero y el espacio no pierde dimensiones.

#### B. Espacio Columna (*Column Space*)
El **espacio columna** de una matriz $A$ es el conjunto de todos los vectores resultantes que se pueden alcanzar mediante su transformación lineal.
*   **La conexión con las columnas:**
    Recordemos que las columnas de la matriz $A$ son los vectores donde aterrizan los vectores base ($\hat{i}$ y $\hat{j}$). Por lo tanto, cualquier vector resultante de la transformación es una combinación lineal de estas columnas.
*   **Definición formal:**
    El espacio columna es el **espacio generado (*span*)** por las columnas de la matriz $A$.
*   **Relación con el Rango:**
    El rango es precisamente la dimensión del espacio columna.
*   **Relación con los Sistemas de Ecuaciones:**
    El sistema $A\vec{x} = \vec{v}$ tiene solución si y solo si el vector destino $\vec{v}$ pertenece al espacio columna de $A$ ($\vec{v} \in \text{Col}(A)$).

---

### 5. Núcleo o Espacio Nulo (*Null Space / Kernel*)
Cuando una transformación lineal colapsa el espacio a una dimensión menor, muchos vectores son arrastrados y aplastados directamente sobre el origen (el vector cero, $\vec{0}$).

*   **Definición:**
    El **espacio nulo** (también llamado **núcleo** o *kernel*) de una matriz $A$ es el conjunto de todos los vectores de entrada $\vec{x}$ que aterrizan exactamente en el origen tras aplicar la transformación:
    $$A\vec{x} = \vec{0}$$

#### Significado para los Sistemas de Ecuaciones:
*   Si $\det(A) \neq 0$ (rango completo), el único vector que aterriza en el origen es el propio vector cero $\vec{0}$. El espacio nulo contiene únicamente al vector cero $\{\vec{0}\}$ (dimensión 0).
*   Si el espacio colapsa (por ejemplo, de 2D a una línea), habrá una línea entera de vectores que aterricen en el origen. En este caso, el espacio nulo es esa línea recta que pasa por el origen (dimensión 1).
*   Si tenemos una solución a $A\vec{x} = \vec{v}$ y sumamos cualquier vector del espacio nulo ($\vec{x}_0$ tal que $A\vec{x}_0 = \vec{0}$), obtenemos otra solución válida:
    $$A(\vec{x} + \vec{x}_0) = A\vec{x} + A\vec{x}_0 = \vec{v} + \vec{0} = \vec{v}$$
    Esto explica geométricamente por qué un espacio nulo no trivial da lugar a infinitas soluciones cuando el vector de destino está dentro de la imagen (espacio columna).

---

> [!IMPORTANT]
> **Resumen de Conceptos Geométricos:**
> *   **$A\vec{x} = \vec{v}$**: ¿Qué vector $\vec{x}$ aterriza en $\vec{v}$?
> *   **$A^{-1}$**: La transformación que deshace $A$. Solo existe si el determinante es diferente de cero.
> *   **Espacio Columna**: El conjunto de todas las posibles salidas (el *span* de las columnas de $A$).
> *   **Rango**: La dimensión del espacio columna.
> *   **Espacio Nulo (Núcleo)**: El conjunto de vectores de entrada que la transformación envía al origen $\vec{0}$.

---

## Capítulo 8: Matrices no cuadradas como transformaciones entre dimensiones (*Nonsquare matrices as transformations between dimensions*)

Este capítulo explora cómo interpretar las matrices no cuadradas (por ejemplo, de $3 \times 2$ o $2 \times 3$) como transformaciones lineales que mueven vectores entre espacios de diferentes dimensiones.

### 1. Transformaciones entre Dimensiones
Hasta ahora, hemos visto matrices cuadradas ($2 \times 2$ o $3 \times 3$) que toman vectores de entrada de una dimensión determinada y devuelven vectores en esa misma dimensión. Sin embargo, es perfectamente posible realizar una transformación lineal que cambie la dimensionalidad del espacio.

Al igual que en los casos anteriores, una transformación es lineal si mantiene las líneas de la cuadrícula rectas, paralelas y uniformemente espaciadas, y mantiene el origen fijo. Para definirla por completo, **solo necesitamos rastrear dónde aterrizan los vectores base del espacio de entrada**.

---

### 2. Regla de Dimensiones: Filas y Columnas
Para una matriz de tamaño $m \times n$:
*   **Número de Columnas ($n$):** Indica la **dimensión del espacio de entrada**. Esto es porque necesitamos una columna para registrar el destino de cada uno de los $n$ vectores base de la entrada (por ejemplo, 2 vectores base en 2D, o 3 en 3D).
*   **Número de Filas ($m$):** Indica la **dimensión del espacio de salida**. Cada vector de destino en las columnas debe expresarse con $m$ coordenadas correspondientes a la dimensión del espacio donde aterriza.

> [!IMPORTANT]
> Una matriz de **$m \times n$** representa una transformación lineal que mapea vectores del espacio **$n$-dimensional** (entrada) al espacio **$m$-dimensional** (salida):
> $$T: \mathbb{R}^n \to \mathbb{R}^m$$

---

### 3. Casos y Ejemplos Comunes

#### A. Transformación de 2D a 3D (Matriz de $3 \times 2$)
Una matriz de $3 \times 2$ toma un vector bidimensional de entrada y devuelve un vector tridimensional de salida.

*   **Matriz:**
    $$A = \begin{bmatrix} a & b \\ c & d \\ e & f \end{bmatrix}$$
*   **Significado de las Columnas:**
    *   La primera columna es el destino en 3D del vector base $\hat{i}$ de 2D: $\hat{i}_{\text{transformado}} = \begin{bmatrix} a \\ c \\ e \end{bmatrix}$.
    *   La segunda columna es el destino en 3D del vector base $\hat{j}$ de 2D: $\hat{j}_{\text{transformado}} = \begin{bmatrix} b \\ d \\ f \end{bmatrix}$.
*   **Multiplicación por un Vector:**
    $$A \begin{bmatrix} x \\ y \end{bmatrix} = x \begin{bmatrix} a \\ c \\ e \end{bmatrix} + y \begin{bmatrix} b \\ d \\ f \end{bmatrix} = \begin{bmatrix} ax + by \\ cx + dy \\ ex + fy \end{bmatrix}$$

> [!NOTE]
> **Interpretación Geométrica:**
> Geométricamente, esta transformación toma el plano 2D de entrada y lo "mapea" (o incrusta) dentro del espacio tridimensional 3D. El **espacio columna** de esta matriz es un plano bidimensional que pasa por el origen en el espacio 3D. Dado que las dos columnas son vectores en 3D, su rango máximo es 2 (el espacio resultante es un plano 2D).

#### B. Transformación de 3D a 2D (Matriz de $2 \times 3$)
Una matriz de $2 \times 3$ toma un vector tridimensional de entrada y devuelve un vector bidimensional de salida.

*   **Matriz:**
    $$B = \begin{bmatrix} a & b & c \\ d & e & f \end{bmatrix}$$
*   **Significado de las Columnas:**
    *   La primera columna es el destino en 2D de $\hat{i}$: $\hat{i}_{\text{transformado}} = \begin{bmatrix} a \\ d \end{bmatrix}$.
    *   La segunda columna es el destino en 2D de $\hat{j}$: $\hat{j}_{\text{transformado}} = \begin{bmatrix} b \\ e \end{bmatrix}$.
    *   La tercera columna es el destino en 2D de $\hat{k}$: $\hat{k}_{\text{transformado}} = \begin{bmatrix} c \\ f \end{bmatrix}$.
*   **Multiplicación por un Vector:**
    $$B \begin{bmatrix} x \\ y \\ z \end{bmatrix} = x \begin{bmatrix} a \\ d \end{bmatrix} + y \begin{bmatrix} b \\ e \end{bmatrix} + z \begin{bmatrix} c \\ f \end{bmatrix} = \begin{bmatrix} ax + by + cz \\ dx + ey + fz \end{bmatrix}$$

> [!NOTE]
> **Interpretación Geométrica:**
> Esta transformación toma el espacio tridimensional 3D de entrada y lo "proyecta" (o aplasta) sobre el plano bidimensional 2D. El rango máximo de esta matriz es 2 (la dimensión de la salida), lo que significa que necesariamente colapsamos al menos una dimensión de la entrada.

#### C. Transformación de 2D a 1D (Matriz de $1 \times 2$)
Una matriz de $1 \times 2$ (una sola fila con dos columnas) toma un vector bidimensional de entrada y devuelve un único número real (1D).

*   **Matriz:**
    $$C = \begin{bmatrix} a & b \end{bmatrix}$$
*   **Significado de las Columnas:**
    *   $\hat{i}$ aterriza en el número real $a$.
    *   $\hat{j}$ aterriza en el número real $b$.
*   **Multiplicación por un Vector:**
    $$C \begin{bmatrix} x \\ y \end{bmatrix} = x \cdot a + y \cdot b = ax + by$$

> [!TIP]
> **Conexión con el Producto Escalar:**
> Esta operación de tomar un vector en 2D y transformarlo en un número real mediante coeficientes fijos ($ax + by$) es idéntica a la fórmula del **producto escalar (dot product)** entre el vector $\begin{bmatrix} x \\ y \end{bmatrix}$ y el vector $\begin{bmatrix} a \\ b \end{bmatrix}$. Esta equivalencia geométrica y matemática es una muestra del principio de **dualidad** en álgebra lineal, que se detallará en el próximo capítulo.

---

### 4. Diferencias Clave con las Matrices Cuadradas
*   **No existe el Determinante:** El determinante mide cómo cambian las áreas (en 2D) o volúmenes (en 3D) dentro del mismo espacio. Puesto que las dimensiones de entrada y salida son diferentes, no tiene sentido físico ni matemático hablar de un factor de escala de área o volumen común. El determinante **solo está definido para matrices cuadradas**.
*   **No son Invertibles:** Al haber un cambio en la dimensión, se produce una pérdida de información (por ejemplo, al proyectar de 3D a 2D, infinitos puntos colapsan en uno solo) o una restricción dimensional (al mapear de 2D a 3D, no podemos cubrir todo el volumen 3D). Por tanto, no se puede definir una matriz inversa directa que revierta el proceso de forma única.






