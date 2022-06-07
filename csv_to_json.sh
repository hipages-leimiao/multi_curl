jq --slurp --raw-input \
    'split("\n") | .[1:] | map(split(","))| map(select(length > 0)) |
      map({
         "external_id": .[0],
         "stripe_account_status": .[1]|rtrimstr("\r")
      })' \
    attributes.csv | tee attributes.json
