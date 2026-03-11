# RMS Liveness Analysis

## 1. Notation

- $G$ is a proposition, the current Guard.
- $L$ is a set of name/guard pairs $(n, g)$ meaning $n$ is live when $g$ is true
- At a use-site:
  - $n$ is **definitely-live** if $G \implies g$ for some $(n, g) \in L$
  - $n$ is **definitely-dead** if $G \implies \neg g$ for some $(n, g) \in L$
  - $n$ is **maybe-live** if $G \implies g$ or $G \implies \neg g$ cannot be derived.
- Probability:
  - $P_{k,n}(x)$ is the guard of block $k$ arm $n$. It can be derived if $x \geq 100$.
  - $P_{k,n}(0) = \bot$ and $P_{k,n}(100) = \top$.
  - $P_{k,1}(x_1) \lor ... \lor P_{k,n}(x_n) = P_{k,1..n}(\sum_i^n x_i)$
  - Lemma: $(P_{k,1}(x_1) \land g) \lor (P_{k,2}(x_2) \land g) = g$ iff $x_1 + x_2 \geq 100$
- $L, G \vdash \bar{S} \implies L'$ set $L$ is updated to $L'$ when analysing statements $\bar{S}$ along with the current guard $G$ (read as $L, G$ yield $\bar{S}$ implies $L'$).
- $$\begin{array}{rc}
  {\tt (rmsLaCase)} & \begin{array}{c}
  \begin{array}{cc} C_1 & C_2 \end{array}
  \\ \hline
  S_1
  \end{array}
  \end{array}
  $$

  is read as $C_1 \land C_2 \implies S_1$

## 2. Liveness Analysis for Statements

### 2.1. Sequence

$$
\begin{array}{rc}
    {\tt (rmsLaSeq)} & \begin{array}{c}
        \begin{array}{ccc}
            L, G \vdash S \implies L'
            & L', G \vdash \bar{S} \implies L''
        \end{array}
        \\ \hline
        L, G \vdash S \bar{S} \implies L''
    \end{array}
\end{array}
$$

### 2.2. Include DRS

$$
\begin{array}{rc}
    {\tt (rmsLaIncDrs)} & \begin{array}{c}
        \begin{array}{ccc}
            {\tt \#include\_drs\ PATH}
            & {\tt PATH} \vdash \bar{S}
            & L, G \vdash \bar{S} \implies L'
        \end{array}
        \\ \hline
        L, G \vdash {\tt \#include\_drs\ PATH} \implies L'
    \end{array}
\end{array}
$$

### 2.3. Include XS

$$
\begin{array}{rc}
    {\tt (rmsLaIncXs)} & \begin{array}{c}
        \begin{array}{ccc}
            {\tt \#includeXS\ PATH}
            & {\tt PATH} \vdash \bar{S}
        \end{array}
        \\ \hline
        L, G \vdash {\tt \#includeXS\ PATH} \implies L
    \end{array}
\end{array}
$$

Note: including XS files has no effect on the RMS file's namespace

### 2.4. Const/Label Definitions

$$
\begin{array}{rc}
    {\tt (rmsLaConstDef)} & \begin{array}{c}
        L, G \vdash {\tt \#const\ NAME\ VALUE} \implies L \oplus ({\tt NAME}, G)
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (rmsLaLabelDef)} & \begin{array}{c}
        L, G \vdash {\tt \#define\ NAME} \implies L \oplus ({\tt NAME}, G)
    \end{array}
\end{array}
$$

### 2.5. If-Else

$$
\begin{array}{rc}
    {\tt (rmsLaIfElse)} & \begin{array}{c}
        \begin{array}{c}
            L, G \land C_1 \vdash S_1 \implies L_1
            \\ L, G \land \neg C_1 \land ... \land \neg C_{i-1} \land C_i \vdash S_i \implies L_i
            \\ L, G \land \neg C_1 \land ... \land \neg C_n \vdash S_e \implies L_e
        \end{array}
        \\ \hline
        L, G \vdash {\tt if}\ C_1\ \bar{S_1}\ {\tt elseif}\ C_2\ \bar{S_2}\ ... \ {\tt elseif}\ C_n\ \bar{S_n}\ {\tt else}\ \bar{S_e} \implies L\ \cup\ \{(n, g_1 \lor ... \lor g_k)\ |\ \forall\ i, (n, g_i) \in L_i \backslash L \}
    \end{array}
\end{array}
$$

### 2.5. Random

$$
\begin{array}{rc}
    {\tt (rmsLaRandom)} & \begin{array}{c}
        \begin{array}{c}
            L, G \land P(p_i) \vdash S_i \implies L_i\
        \end{array}
        \\ \hline
        L, G \vdash {\tt start\_random}\ {\tt percent\_chance\ p_1}\ \bar{S_1}\ ...\ {\tt percent\_chance\ p_n}\ \bar{S_n}\ {\tt end\_random} \implies L\ \cup\ \{(n, g_1 \lor ... \lor g_k)\ |\ \forall\ i, (n, g_i) \in L_i \backslash L \}
\end{array}
\end{array}
$$

#### Example 1
```rms
start_random
    percent_chance 50 #define A
    percent_chance 50 #define A
end_random
```


From the two arms of this block we get:
- $L_1 = \{(A, P_{1,1}(50))\}$
- $L_2 = \{(A, P_{1,2}(50))\}$

Which join into $L = \{(A, P_{1,1}(50) \lor P_{1,2}(50))\} = \{(A, \top)\}$ 

#### Example 2
```rms
start_random
    percent_chance 50 #define A
    percent_chance 50 #define B
end_random

start_random
    percent_chance 50
        if A
            #define NAME1
            #define NAME2
        endif
    percent_chance 50
        if A
            #define NAME1
        endif
        if B
            #define NAME2
        endif
end_random
```


From the two arms of the first block we get:
- $L_1 = \{(A, P_{1,1}(50))\}$
- $L_2 = \{(B, P_{1,2}(50))\}$

Which join into $L = \{(A, P_1(50)), (B, P_1(50))\}$

Then the arms of the subsequent blocks give us:
- $L_1 = \{({\tt NAME1}, P_{2,1}(50) \land A), ({\tt NAME2}, P_{2,1}(50) \land A)\}$
- $L_2 = \{({\tt NAME1}, P_{2,2}(50) \land A), ({\tt NAME2}, P_{2,2}(50) \land B)\}$

which finally join to give us $L = \{(A, P_1(50)), (B, P_1(50)), ({\tt NAME1}, (P_{2,1}(50) \land A) \lor (P_{2,2}(50) \land A)), ({\tt NAME2}, (P_{2,1}(50) \land A) \lor (P_{2,2}(50) \land B))\}$

which simplifies to $L = \{(A, P_1(50)), (B, P_1(50)), ({\tt NAME1}, A), ({\tt NAME2}, (P_{2,1}(50) \land A) \lor (P_{2,2}(50) \land B))\}$

### 2.6. Section Start

$$
\begin{array}{rc}
    {\tt (rmsLaSection)} & \begin{array}{c}
        \begin{array}{ccc}
            \texttt{$<$SECTION$>$}
        \end{array}
        \\ \hline
        L, G \vdash \texttt{$<$SECTION$>$} \implies L
    \end{array}
\end{array}
$$

### 2.7. Command

$$
\begin{array}{rc}
    {\tt (rmsLaCmd)} & \begin{array}{c}
        \begin{array}{ccc}
            {\tt command\ \bar{A}}
        \end{array}
        \\ \hline
        L, G \vdash {\tt command\ \bar{A}} \implies L
    \end{array}
\end{array}
$$

### 2.8. Block

$$
\begin{array}{rc}
    {\tt (rmsLaBlock)} & \begin{array}{c}
        \begin{array}{ccc}
        L, G \vdash \bar{S} \implies L'
        \end{array}
        \\ \hline
        L, G \vdash \{\ \bar{S}\ \} \implies L'
    \end{array}
\end{array}
$$