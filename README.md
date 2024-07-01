# Python to Mermaid converter

Convert your python module into a Markdown file with Mermaid flowcharts of containing functions.

# Install

There's no pre-built package yet. However you can install it with cargo easily:

```
cargo install python-to-mermaid
```

# Usage

This is very simple program that takes a python file as a stdin and outputs a markdown file.

```
python-to-mermaid < python_file.py > output.md
```

# Example

Given the following python code:

```python
import random

def get_random_number():
    return random.randint(0, 100)

def guess_number(target):
    while True:
        guess = int(input("Guess a number between 0 and 100: "))
        if guess == target:
            return 1
        elif guess < target:
            print("Too low!")
        else:
            print("Too high!")

def play_game():
    target = get_random_number()
    attempts = 0
    while True:
        attempts += guess_number(target)
        if attempts == 1:
            break
    print(f"Congratulations! You guessed the number {target} in {attempts} attempt(s).")

play_game()
```

The following markdown file will be generated:

````markdown
## `get_random_number`

```mermaid
flowchart TD;
A("Begin: get_random_number");
B("End: get_random_number");
C("return: random.randint(0, 100)");
A --> C;
C --> B;
```

## `guess_number`

```mermaid
flowchart TD;
A("Begin: guess_number");
B("End: guess_number");
subgraph "Begin: while True"
C[/"Begin: while True"\];
D[\"End: while True"/];
E["guess = int(input('Guess a number between 0 and 100: '))"];
F{"if: guess == target ?"};
G("return: 1");
H{"if: guess < target ?"};
I["print('Too low!')"];
J["print('Too high!')"];
end
D --> B;
A --> C;
C --> E;
E --> F;
F -->|"T"| G;
F -->|"F"| H;
H -->|"T"| I;
H -->|"F"| J;
G --> D;
I --> D;
J --> D;
```

## `play_game`

```mermaid
flowchart TD;
A("Begin: play_game");
B("End: play_game");
C["target = get_random_number()"];
D["attempts = 0"];
J["print(f'Congratulations! You guessed the number {target} in {attempts} attempt(s).')"];
subgraph "Begin: while True"
E[/"Begin: while True"\];
F[\"End: while True"/];
G["attempts += guess_number(target)"];
H{"if: attempts == 1 ?"};
I["break"];
end
A --> C;
C --> D;
F --> J;
J --> B;
D --> E;
E --> G;
G --> H;
H -->|"T"| I;
I --> F;
H -->|"F"| F;
```
````
