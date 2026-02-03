for feature in string compact-str smartstring arraystring-4 arraystring-20
do
  rm -rf target/criterion/
  cargo bench -p helix-tui --bench tui --no-default-features --features "$feature-symbol" -- --measurement-time 10
  ./make_json.sh target/criterion "$feature-report.json"
done
