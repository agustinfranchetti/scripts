# Example aliases

#cleans ports 3000, 3001, 3002, 3003
alias clean-ports='lsof -i :3000,3001,3002,3003 | grep LISTEN | awk '\''{print $2}'\'' | xargs kill'
if command -v pyenv 1>/dev/null 2>&1; then
  eval "$(pyenv init -)"
fi

#alias for organize downloads script
alias organize-downloads='/Users/agustin/Workspace/personal/scripts/organize-downloads'