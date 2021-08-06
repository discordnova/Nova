# Developement environment

Nova utilise Bazel comme gestionnaire de build. Bazel nous permet de combiner plusieurs langages (Rust, Go, ...) dans une seule pipeline tout en profitant de la compilation
incrémentielle et de la compilation distante.

Cependant, certaines démarches sont a suivre pour mettre a jour les dépendances des programmes. Nova utilise Gazelle pour générer les fichiers "BUILD" des projets Go et Cargo-Raze pour générer des fichiers "BUILD" contenant toutes les dépendances.

Quand vous éditez les dépendances d'un projet, vous devez éxécuter une commande pour mettre a jour ces fichiers.

# Rust

Pour mettre a jour ou créer un projet Rust, utilisez cette commande dans le dossier de votre projet.

```
bazel run @cargo_raze//:raze -- --manifest-path=$(realpath Cargo.toml)
```

Si c'est un nouveau projet, assurez-vous d'avoir configuré raze dans votre Cargo.toml et de l'avoir rajouté aux sources dans le fichier WORKSPACE.

# Golang

Pour mettre a jour les dépendances des projets go, exécutez 

```
bazel run //:gazelle
```