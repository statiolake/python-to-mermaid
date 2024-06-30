## is_linux_musl

```mermaid
flowchart TD;
A("Begin: is_linux_musl");
B["Assign"];
C["Return"];
D("End: is_linux_musl");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## Platform.detect

```mermaid
flowchart TD;
A("Begin: Platform.detect");
B["Assign"];
C{"If"};
D["Return"];
E{"If"};
F["Return"];
G{"If"};
H{"If"};
I["Return"];
J["Return"];
K["Raise"];
L("End: Platform.detect");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
D -->|""| F;
C -->|"F"| F;
F -->|""| G;
E -->|"F"| G;
F -->|""| H;
E -->|"F"| H;
F -->|""| I;
E -->|"F"| I;
F -->|""| J;
E -->|"F"| J;
J -->|""| K;
G -->|"F"| K;
K -->|""| L;

```

## Arch.detect

```mermaid
flowchart TD;
A("Begin: Arch.detect");
B["Assign"];
C{"If"};
D["Return"];
E{"If"};
F["Return"];
G["Raise"];
H("End: Arch.detect");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
D -->|""| F;
C -->|"F"| F;
F -->|""| G;
E -->|"F"| G;
G -->|""| H;

```

## Directories.Source.__init__

```mermaid
flowchart TD;
A("Begin: Directories.Source.__init__");
B["Assign"];
C["Assign"];
D["Assign"];
E["Assign"];
F["Assign"];
G("End: Directories.Source.__init__");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;

```

## Directories.Target.__init__

```mermaid
flowchart TD;
A("Begin: Directories.Target.__init__");
B["Assign"];
C["Assign"];
D["Assign"];
E["Assign"];
F["Assign"];
G{"If"};
H["Assign"];
I["Assign"];
J("End: Directories.Target.__init__");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;
F -->|""| H;
H -->|""| I;
G -->|"F"| I;
I -->|""| J;

```

## Directories.Target._get_nvimfiles

```mermaid
flowchart TD;
A("Begin: Directories.Target._get_nvimfiles");
B{"If"};
C["Assign"];
D["Return"];
E["Return"];
F("End: Directories.Target._get_nvimfiles");
A -->|""| B;
A -->|""| C;
A -->|""| D;
A -->|""| E;
D -->|""| F;
E -->|""| F;

```

## Directories.Target._get_pwshprofiles

```mermaid
flowchart TD;
A("Begin: Directories.Target._get_pwshprofiles");
B["Assign"];
C["Assign"];
D["Assign"];
E{"If"};
F["Expr"];
G["Expr"];
H["Assign"];
I{"If"};
J["Expr"];
K["Expr"];
L["Return"];
M("End: Directories.Target._get_pwshprofiles");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
D -->|""| F;
D -->|""| G;
G -->|""| H;
E -->|"F"| H;
H -->|""| I;
H -->|""| J;
H -->|""| K;
K -->|""| L;
I -->|"F"| L;
L -->|""| M;

```

## Directories.__init__

```mermaid
flowchart TD;
A("Begin: Directories.__init__");
B["Expr"];
C["Assign"];
D["Assign"];
K("End: Directories.__init__");
A -->|""| B;
B -->|""| C;
C -->|""| D;
J -->|""| K;
E("Begin: For");
F["Expr"];
G("End: For");
D -->|""| E;
E -->|""| F;
F -->|""| G;
H("Begin: For");
I["Expr"];
J("End: For");
G -->|""| H;
H -->|""| I;
I -->|""| J;

```

## Directories.envvar

```mermaid
flowchart TD;
A("Begin: Directories.envvar");
B["Assign"];
C{"If"};
D["Raise"];
E["Return"];
F("End: Directories.envvar");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
E -->|""| F;

```

## remove_all

```mermaid
flowchart TD;
A("Begin: remove_all");
B["Expr"];
C["Expr"];
D{"If"};
E["Return"];
F["Expr"];
G("End: remove_all");
A -->|""| B;
B -->|""| C;
C -->|""| D;
C -->|""| E;
E -->|""| F;
D -->|"F"| F;
F -->|""| G;

```

## remove_all.force_remove_readonly

```mermaid
flowchart TD;
A("Begin: remove_all.force_remove_readonly");
B["Expr"];
C["Expr"];
D("End: remove_all.force_remove_readonly");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## is_link

```mermaid
flowchart TD;
A("Begin: is_link");
B["Assign"];
C{"If"};
D["Return"];
H("End: is_link");
A -->|""| B;
B -->|""| C;
B -->|""| D;
G -->|""| H;
E("Begin: Try");
F["Return"];
G("End: Try");
D -->|""| E;
C -->|"F"| E;
E -->|""| F;
F -->|""| G;

```

## ensure_link_not_exist

```mermaid
flowchart TD;
A("Begin: ensure_link_not_exist");
B["Assign"];
C{"If"};
D["Return"];
E{"If"};
F["Expr"];
G{"If"};
H["Raise"];
I["Return"];
J{"If"};
K["Expr"];
L{"If"};
M["Raise"];
N["Return"];
O["Raise"];
P("End: ensure_link_not_exist");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
D -->|""| F;
C -->|"F"| F;
D -->|""| G;
C -->|"F"| G;
D -->|""| H;
C -->|"F"| H;
D -->|""| I;
C -->|"F"| I;
I -->|""| J;
E -->|"F"| J;
I -->|""| K;
E -->|"F"| K;
I -->|""| L;
E -->|"F"| L;
I -->|""| M;
E -->|"F"| M;
I -->|""| N;
E -->|"F"| N;
N -->|""| O;
J -->|"F"| O;
O -->|""| P;

```

## echo

```mermaid
flowchart TD;
A("Begin: echo");
B["Expr"];
C["Expr"];
D("End: echo");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## confirm

```mermaid
flowchart TD;
A("Begin: confirm");
B{"If"};
C["Return"];
D{"If"};
E["Return"];
F["Assign"];
G{"If"};
H["Return"];
I("End: confirm");
A -->|""| B;
A -->|""| C;
C -->|""| D;
B -->|"F"| D;
C -->|""| E;
B -->|"F"| E;
E -->|""| F;
D -->|"F"| F;
F -->|""| G;
F -->|""| H;
H -->|""| I;
G -->|"F"| I;

```

## linkf

```mermaid
flowchart TD;
A("Begin: linkf");
B["Expr"];
C{"If"};
D["Raise"];
E["Expr"];
F["Expr"];
G("End: linkf");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
E -->|""| F;
F -->|""| G;

```

## linkd

```mermaid
flowchart TD;
A("Begin: linkd");
B["Expr"];
C{"If"};
D["Raise"];
E["Expr"];
F{"If"};
G["Expr"];
H["Return"];
I["Expr"];
J("End: linkd");
A -->|""| B;
B -->|""| C;
B -->|""| D;
D -->|""| E;
C -->|"F"| E;
E -->|""| F;
E -->|""| G;
E -->|""| H;
H -->|""| I;
F -->|"F"| I;
I -->|""| J;

```

## mergef

```mermaid
flowchart TD;
A("Begin: mergef");
J{"If"};
K["Assign"];
L{"If"};
M["Assign"];
N["Raise"];
R("End: mergef");
I -->|""| J;
I -->|""| K;
I -->|""| L;
I -->|""| M;
I -->|""| N;
Q -->|""| R;
B("Begin: With");
C["Assign"];
D("End: With");
A -->|""| B;
B -->|""| C;
C -->|""| D;
E("Begin: Try");
I("End: Try");
D -->|""| E;
H -->|""| I;
F("Begin: With");
G["Assign"];
H("End: With");
E -->|""| F;
F -->|""| G;
G -->|""| H;
O("Begin: With");
P["Expr"];
Q("End: With");
K -->|""| O;
M -->|""| O;
N -->|""| O;
O -->|""| P;
P -->|""| Q;

```

## git_clone

```mermaid
flowchart TD;
A("Begin: git_clone");
B{"If"};
C{"If"};
D["Expr"];
E["Return"];
F["Expr"];
G["Expr"];
H("End: git_clone");
A -->|""| B;
A -->|""| C;
A -->|""| D;
A -->|""| E;
A -->|""| F;
F -->|""| G;
B -->|"F"| G;
G -->|""| H;

```

## download

```mermaid
flowchart TD;
A("Begin: download");
J("End: download");
I -->|""| J;
B("Begin: With");
C["Assign"];
D["Expr"];
E("End: With");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
F("Begin: Try");
G["Assign"];
H["Return"];
I("End: Try");
E -->|""| F;
F -->|""| G;
G -->|""| H;
H -->|""| I;

```

## ungzip

```mermaid
flowchart TD;
A("Begin: ungzip");
B["Return"];
C("End: ungzip");
A -->|""| B;
B -->|""| C;

```

## unzip

```mermaid
flowchart TD;
A("Begin: unzip");
E("End: unzip");
D -->|""| E;
B("Begin: With");
C["Return"];
D("End: With");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## save_as

```mermaid
flowchart TD;
A("Begin: save_as");
B["Expr"];
F("End: save_as");
A -->|""| B;
E -->|""| F;
C("Begin: With");
D["Expr"];
E("End: With");
B -->|""| C;
C -->|""| D;
D -->|""| E;

```

## setup_git

```mermaid
flowchart TD;
A("Begin: setup_git");
B{"If"};
C["Expr"];
D["Expr"];
E{"If"};
F["Expr"];
G["Expr"];
H["Expr"];
I["Expr"];
J("End: setup_git");
A -->|""| B;
A -->|""| C;
A -->|""| D;
C -->|""| E;
D -->|""| E;
C -->|""| F;
D -->|""| F;
F -->|""| G;
E -->|"F"| G;
G -->|""| H;
H -->|""| I;
I -->|""| J;

```

## setup_neovim_appimage

```mermaid
flowchart TD;
A("Begin: setup_neovim_appimage");
B{"If"};
C["Return"];
D["Assign"];
E["Assign"];
F["Expr"];
G["Expr"];
H("End: setup_neovim_appimage");
A -->|""| B;
A -->|""| C;
C -->|""| D;
B -->|"F"| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;
G -->|""| H;

```

## install_node

```mermaid
flowchart TD;
A("Begin: install_node");
B["Assign"];
C{"If"};
D["Expr"];
E["Return"];
F["Assign"];
G{"If"};
H["Assign"];
I["Assign"];
J{"If"};
K["Assign"];
L["Assign"];
M{"If"};
N["Assign"];
O["Assign"];
P["Assign"];
Q{"If"};
R["Assign"];
S["Assign"];
T["Raise"];
U["Assign"];
V["Assign"];
W["Expr"];
X["Assign"];
Y["Expr"];
Z["Expr"];
AG["Expr"];
AK("End: install_node");
A -->|""| B;
B -->|""| C;
B -->|""| D;
B -->|""| E;
E -->|""| F;
C -->|"F"| F;
F -->|""| G;
F -->|""| H;
F -->|""| I;
F -->|""| J;
F -->|""| K;
F -->|""| L;
F -->|""| M;
F -->|""| N;
F -->|""| O;
F -->|""| P;
F -->|""| Q;
F -->|""| R;
F -->|""| S;
F -->|""| T;
I -->|""| U;
L -->|""| U;
P -->|""| U;
S -->|""| U;
T -->|""| U;
U -->|""| V;
V -->|""| W;
W -->|""| X;
X -->|""| Y;
Y -->|""| Z;
AF -->|""| AG;
AJ -->|""| AK;
AA("Begin: With");
AB["Assign"];
AC["Expr"];
AD["Expr"];
AE["Expr"];
AF("End: With");
Z -->|""| AA;
AA -->|""| AB;
AB -->|""| AC;
AC -->|""| AD;
AD -->|""| AE;
AE -->|""| AF;
AH("Begin: Try");
AI["Expr"];
AJ("End: Try");
AG -->|""| AH;
AH -->|""| AI;
AI -->|""| AJ;

```

## install_deno

```mermaid
flowchart TD;
A("Begin: install_deno");
B["Assign"];
C{"If"};
D["Expr"];
E["Return"];
F["Assign"];
G["Assign"];
H{"If"};
I["Assign"];
J{"If"};
K{"If"};
L["Assign"];
M{"If"};
N["Assign"];
O{"If"};
P["Assign"];
Q["Raise"];
R["Assign"];
S["Expr"];
T["Expr"];
U["Assign"];
V["Expr"];
W["Expr"];
X("End: install_deno");
A -->|""| B;
B -->|""| C;
B -->|""| D;
B -->|""| E;
E -->|""| F;
C -->|"F"| F;
F -->|""| G;
G -->|""| H;
G -->|""| I;
G -->|""| J;
G -->|""| K;
G -->|""| L;
G -->|""| M;
G -->|""| N;
G -->|""| O;
G -->|""| P;
G -->|""| Q;
I -->|""| R;
L -->|""| R;
N -->|""| R;
M -->|"F"| R;
P -->|""| R;
Q -->|""| R;
R -->|""| S;
S -->|""| T;
T -->|""| U;
U -->|""| V;
V -->|""| W;
W -->|""| X;

```

## setup_vscode_remote_server

```mermaid
flowchart TD;
A("Begin: setup_vscode_remote_server");
B["Expr"];
C["Expr"];
D("End: setup_vscode_remote_server");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## setup_neovim

```mermaid
flowchart TD;
A("Begin: setup_neovim");
B["Expr"];
C["Expr"];
D["Expr"];
E["Expr"];
F["Expr"];
G["Expr"];
H["Expr"];
I["Expr"];
J["Expr"];
K["Assign"];
L["Expr"];
M["Assign"];
N{"If"};
O["Assign"];
P{"If"};
Q["Assign"];
R["Expr"];
S["Expr"];
T["Assign"];
U{"If"};
V["Expr"];
W["Expr"];
AA("End: setup_neovim");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;
G -->|""| H;
H -->|""| I;
I -->|""| J;
J -->|""| K;
K -->|""| L;
L -->|""| M;
M -->|""| N;
M -->|""| O;
O -->|""| P;
N -->|"F"| P;
O -->|""| Q;
N -->|"F"| Q;
O -->|""| R;
N -->|"F"| R;
R -->|""| S;
P -->|"F"| S;
S -->|""| T;
T -->|""| U;
T -->|""| V;
V -->|""| W;
U -->|"F"| W;
Z -->|""| AA;
X("Begin: Try");
Y["Expr"];
Z("End: Try");
W -->|""| X;
X -->|""| Y;
Y -->|""| Z;

```

## setup_vscode_neovim

```mermaid
flowchart TD;
A("Begin: setup_vscode_neovim");
B["Expr"];
C["Expr"];
D["Expr"];
E["Expr"];
F("End: setup_vscode_neovim");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;

```

## setup_emacs

```mermaid
flowchart TD;
A("Begin: setup_emacs");
B["Assign"];
C["Expr"];
D["Expr"];
E("End: setup_emacs");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;

```

## setup_powershell

```mermaid
flowchart TD;
A("Begin: setup_powershell");
B["Assign"];
C["Assign"];
L("End: setup_powershell");
A -->|""| B;
B -->|""| C;
K -->|""| L;
D("Begin: For");
E["Expr"];
K("End: For");
C -->|""| D;
D -->|""| E;
J -->|""| K;
F("Begin: With");
G["Assign"];
H["Expr"];
I["Expr"];
J("End: With");
E -->|""| F;
F -->|""| G;
G -->|""| H;
H -->|""| I;
I -->|""| J;

```

## setup_alacritty

```mermaid
flowchart TD;
A("Begin: setup_alacritty");
B["Assign"];
C["Expr"];
D{"If"};
E["Expr"];
F{"If"};
G["Expr"];
H("End: setup_alacritty");
A -->|""| B;
B -->|""| C;
C -->|""| D;
C -->|""| E;
C -->|""| F;
C -->|""| G;
E -->|""| H;
G -->|""| H;
F -->|"F"| H;

```

## setup_kitty

```mermaid
flowchart TD;
A("Begin: setup_kitty");
B["Expr"];
C("End: setup_kitty");
A -->|""| B;
B -->|""| C;

```

## setup_bash

```mermaid
flowchart TD;
A("Begin: setup_bash");
B["Expr"];
C["Expr"];
D("End: setup_bash");
A -->|""| B;
B -->|""| C;
C -->|""| D;

```

## setup_zsh

```mermaid
flowchart TD;
A("Begin: setup_zsh");
B["Expr"];
C["Expr"];
D["Expr"];
E["Expr"];
F["Expr"];
G("End: setup_zsh");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;

```

## setup_input_remapper

```mermaid
flowchart TD;
A("Begin: setup_input_remapper");
B["Expr"];
C("End: setup_input_remapper");
A -->|""| B;
B -->|""| C;

```

## setup_tmux

```mermaid
flowchart TD;
A("Begin: setup_tmux");
B["Expr"];
C("End: setup_tmux");
A -->|""| B;
B -->|""| C;

```

## setup_karabiner

```mermaid
flowchart TD;
A("Begin: setup_karabiner");
B["Expr"];
C["Expr"];
D["Expr"];
E("End: setup_karabiner");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;

```

## setup_scripts

```mermaid
flowchart TD;
A("Begin: setup_scripts");
B["Expr"];
C["Expr"];
D{"If"};
H["Expr"];
I["Expr"];
J{"If"};
N["Expr"];
O("End: setup_scripts");
A -->|""| B;
B -->|""| C;
C -->|""| D;
C -->|""| H;
G -->|""| I;
H -->|""| I;
I -->|""| J;
I -->|""| N;
M -->|""| O;
N -->|""| O;
E("Begin: With");
F["Expr"];
G("End: With");
C -->|""| E;
E -->|""| F;
F -->|""| G;
K("Begin: With");
L["Expr"];
M("End: With");
I -->|""| K;
K -->|""| L;
L -->|""| M;

```

## main

```mermaid
flowchart TD;
A("Begin: main");
B["Assign"];
C["Expr"];
D["Expr"];
E["Expr"];
F["Expr"];
G["Expr"];
H["Expr"];
I["Expr"];
J["Expr"];
K["Expr"];
L["Expr"];
M["Expr"];
N["Expr"];
O["Expr"];
P["Expr"];
Q{"If"};
R["Expr"];
S["Expr"];
T{"If"};
U["Expr"];
V["Expr"];
W["Expr"];
X["Expr"];
Y["Expr"];
Z["Expr"];
AA{"If"};
AB["Expr"];
AC["Expr"];
AD["Expr"];
AE{"If"};
AF["Expr"];
AG["Expr"];
AH{"If"};
AI["Expr"];
AJ["Expr"];
AK{"If"};
AL["Expr"];
AM["Expr"];
AN["Expr"];
AO["Expr"];
AP("End: main");
A -->|""| B;
B -->|""| C;
C -->|""| D;
D -->|""| E;
E -->|""| F;
F -->|""| G;
G -->|""| H;
H -->|""| I;
I -->|""| J;
J -->|""| K;
K -->|""| L;
L -->|""| M;
M -->|""| N;
N -->|""| O;
O -->|""| P;
P -->|""| Q;
P -->|""| R;
P -->|""| S;
S -->|""| T;
Q -->|"F"| T;
S -->|""| U;
Q -->|"F"| U;
S -->|""| V;
Q -->|"F"| V;
V -->|""| W;
T -->|"F"| W;
W -->|""| X;
X -->|""| Y;
Y -->|""| Z;
Z -->|""| AA;
Z -->|""| AB;
Z -->|""| AC;
Z -->|""| AD;
AD -->|""| AE;
AA -->|"F"| AE;
AD -->|""| AF;
AA -->|"F"| AF;
AD -->|""| AG;
AA -->|"F"| AG;
AG -->|""| AH;
AE -->|"F"| AH;
AG -->|""| AI;
AE -->|"F"| AI;
AG -->|""| AJ;
AE -->|"F"| AJ;
AJ -->|""| AK;
AH -->|"F"| AK;
AJ -->|""| AL;
AH -->|"F"| AL;
AL -->|""| AM;
AK -->|"F"| AM;
AM -->|""| AN;
AN -->|""| AO;
AO -->|""| AP;

```

