#include <stdio.h>
#include <string.h>

int main(){
    char my_string[] = "Let's Get Rusty!";
    printf("'my_string' is = '%s'\n", my_string);
    printf("Address of 'my_string' is = '%p'\n", &my_string);
    printf("Length of 'my_string' is = '%lu'\n", strlen(my_string));
    printf("'sizeof()' of 'my_string' is = '%lu' bytes\n", sizeof(my_string));
    printf("\n");

    char *string_ptr = my_string;
    printf("'string_ptr' is = '%s'\n", string_ptr);
    printf("Pointer Address of 'string_ptr' is = '%p'\n", &string_ptr);
    printf("Length of Pointer Address 'string_ptr' is = '%lu'\n", strlen(string_ptr));
    printf("'sizeof()' of Pointer Address 'string_ptr' is = '%lu' bytes\n", sizeof(string_ptr));
    printf("\n");

    /* char buffer[16]; */
    char buffer[sizeof(my_string)];
    strcpy(buffer, my_string);
    printf("The copied string is: %s\n", buffer);
    printf("Length of 'buffer' is = '%lu'\n", strlen(buffer));
    printf("'sizeof()' of 'buffer' is = '%lu' bytes\n", sizeof(buffer));

    return 0;
}
