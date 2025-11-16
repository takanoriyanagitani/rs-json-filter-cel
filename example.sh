#!/bin/sh

echo example 1 using native bin...
jq -c -n '[
	{time:"2025-11-11T00:54:04.012345Z", severity:"INFO", status:200, body:"apt update done"},
	{time:"2025-11-12T00:54:04.012345Z", severity:"WARN", status:500, body:"apt update failure"},
	{time:"2025-11-13T00:54:04.012345Z", severity:"INFO", status:200, body:"apt update done"}
]' |
	jq -c '.[]' |
	./json-filter-cel \
		--json-obj-name 'item' \
		'
			(item.status >= 200)
			&& (0 < size(item.body))
			&& (item.severity.startsWith("I"))
		'
	
echo
echo example 2 using wazero...
jq -c -n '[
	{time:"2025-11-11T00:54:04.012345Z", severity:"INFO", status:200, body:"apt update done"},
	{time:"2025-11-12T00:54:04.012345Z", severity:"WARN", status:500, body:"apt update failure"},
	{time:"2025-11-13T00:54:04.012345Z", severity:"INFO", status:200, body:"apt update done"}
]' |
	jq -c '.[]' |
	wazero run ./json-filter-cel.wasm \
		--json-obj-name 'item' \
		'
			(item.status >= 200)
			&& (0 < size(item.body))
			&& (item.severity.startsWith("I"))
		'
	
