{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  #nativeBuildInputs = with pkgs; [];  # only needed for developing nix/nixos
  buildInputs = with pkgs; [
    # common build inputs
    direnv
    duf
    eza
    fd
    rustup
    xclip
    # project-specific build inputs
    #bun
    #deno
    #nodejs
    #nodePackages.pnpm
  ];
  env = {
    #DENO_BIN="${pkgs.deno}/bin/deno";
    #NODE_BIN="${pkgs.nodejs}/bin/nodejs";
  };
  shellHook = ''
    #alias ls=eza
    #alias ls="ls -alh --color=auto"
    alias ls="eza --long --group --header -a --classify --links --level=3 --color=auto --sort=type --time-style=long-iso --extended"
    alias find=fd
    alias fd="fd --hidden --list-details --color=auto" # cannot be aliased to 'find' if using hlissner doom emacs
    #alias fd="fd --hidden --no-ignore --follow --list-details --color=auto" # cannot be aliased to 'find' if using hlissner doom emacs
    #alias fd="find -L" # cannot be aliased if using hlissner doom emacs
    alias du="duf"
 
    #git
    alias gst="git status"
    alias gc="git commit"
    alias gcm="git commit -m"
    alias ga="git add"
    alias gaa="git add --all"
    alias gcl="git clone -v --progress"
    alias gb="git branch"
    alias gp="git push -u"
    alias gpu="git push -u"
 
    #import parent shell config
    [ -x ~/.bashrc ] && source ~/.bashrc
    [ -x ~/.zshrc ] && source ~/.zshrc
  '';
}