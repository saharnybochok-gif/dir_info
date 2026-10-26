# dir_info

Консольная утилита на Rust для просмотра информации о файлах и папках в директории для Windows.

## Как пользоваться
Скачайте `dir_info.exe` из раздела **Releases**, а потом добавьте в переменную окружения.
Откройте терминал (cmd или PowerShell) в папке с утилитой и напишите (вместо `C:\Windows` вы можете написать другой путь):

```powershell
dir_info C:\Windows\Temp
```

![](image.png)

## Флаги

Чтобы показать версию утилиты, вы можете написать флаг `--version` или `-v`:
```powershell
dir_info --version
```

```powershell
dir_info -v
```
Если вам нужна помощь, вы можете написать флаг `--help` или `-h`:
```powershell
dir_info --help
```

```powershell
dir_info -h
```
