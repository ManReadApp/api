for item in *; do
  if [ "$item" != "crates" ]; then
    git mv "$item" crates/api/
  fi
done
