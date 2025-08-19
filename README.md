# Vault
> Secrets Manager

#### Set up multiple projects
```bash
mkdir project1 && cd project1
vault local
vault add SECRET_ONE=value1

cd ..
mkdir project2 && cd project2  
vault local
vault add SECRET_TWO=value2
```

#### Now manage both from anywhere
```bash
vault add --project project1 NEW_SECRET=hello
vault list --project project1
vault list --project project2
```

##### Load different projects
```bash
eval "$(vault load --project project1 --export)"
echo $SECRET_ONE $NEW_SECRET

eval "$(vault load --project project2 --export)"
echo $SECRET_TWO
```
