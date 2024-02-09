// ===== Warning e erro de dangling pointer ======
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* criar_dangling_pointer() {
    char local[] = "isso é um teste";
    printf("ptr dentro da função: %s\n", local);
    return local; // Erro: retorna uma referência a uma variável local que será desalocada
}

int main() {
    char* ptr = criar_dangling_pointer();
    printf("ptr fora da função: %s\n", ptr);
    // Aqui `ptr` é um dangling pointer, pois aponta para uma memória que foi desalocada
    return 0;
}



// ===== Corrigido ======


// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>

// char* criar_string_segura() {
//     // Aloca memória dinamicamente para a string
//     char* local = (char*)malloc(strlen("isso é um teste") + 1);
//     strcpy(local, "isso é um teste");
//     return local;
// }

// int main() {
//     char* ptr = criar_string_segura();
//     // Imprime o conteúdo apontado por `ptr`
//     printf("%s\n", ptr);

//     // Libera a memória alocada
//     free(ptr);
//     return 0;
// }
