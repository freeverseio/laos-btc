Before deploying the helm charts, please ensure that the appropriate secret to store rpc auth is already created in the namespace.

```yml
apiVersion: v1
kind: Secret
metadata:
  annotations:
  name: bitcoin-rpc-auth
  namespace: laos-bitcoin
type: Opaque
data:
  rpcauth: dGVzdDoyYzE0NzEyZmMzMTM3YTEyZjRiZDdkMGM4YWUxYjg5NSQ5OTBhMmViZTVmN2ViNzg5M2E2ODI5OWM1YTA5YmMyMjI1ODg1NTcwNGFhOGQ3NGEyMWU0ZTAxZTM1YTc3N2Mw
  rpcuser: dGVzdA==
  rpcpassword: dGVzdA==
```
Use this website to generate the appropriate rpcauth string, givern username and password:  https://jlopp.github.io/bitcoin-core-rpc-auth-generator/.
