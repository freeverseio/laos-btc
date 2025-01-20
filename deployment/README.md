Before deploying the helm charts, please ensure that the appropriate secret to store rpc auth is already created in the namespace.

```
apiVersion: v1
kind: Secret
metadata:
  annotations:
  name: bitcoin-rpc-auth
  namespace: laos-bitcoin
type: Opaque
data:
  rpcauth: c3VwZXJzZWNyZXQ=
  rpcuser: c3VwZXJzZWNyZXQ=
  rpcpassowrd: c3VwZXJzZWNyZXQ=
```
