{{- define "decodeSecret" -}}
{{- $secret := (lookup "v1" "Secret" .Release.Namespace "bitcoin-rpc-auth") -}}
{{- if $secret -}}
{{- $value := index $secret.data "rpcauth" | printf "%s" | b64dec -}}
{{- $value -}}
{{- else -}}
{{- printf "ERROR: Secret not found or value missing" -}}
{{- end -}}
{{- end -}}
