﻿/* petclub.c -- use a binary search tree */
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include "tree.h"

char menu(void);
void addpet(Tree* pt);
void droppet(Tree* pt);
void showpets(const Tree* pt);
void findpet(const Tree* pt);
void printitem(Item item);
void uppercase(char* str);

int main(void)
{
   Tree pets;
   char choice;

   initializeTree(&pets);
   while ((choice = menu()) != 'q')
   {
      switch (choice)
      {
      case 'a': addpet(&pets);
         break;
      case 'l': showpets(&pets);
         break;
      case 'f': findpet(&pets);
         break;
      case 'n': printf("%d pets in club\n",
                       treeItemCount(&pets));
         break;
      case 'd': droppet(&pets);
         break;
      default: puts("Switching error");
      }
   }

   deleteAll(&pets);
   puts("Bye.");

   return 0;
}

char menu(void)
{
   int ch;

   puts("Nerfville Pet Club Membership Program");
   puts("Enter the letter corresponding to your choice:");
   puts("a) add a pet          l) show list of pets");
   puts("n) number of pets     f) find pets");
   puts("d) delete a pet       q) quit");
   while ((ch = getchar()) != EOF)
   {
      while (getchar() != '\n')
         /* discard rest of line */;
      ch = tolower(ch);
      if (strchr("alrfndq", ch) == NULL)
         puts("Please enter an a, l, f, n, d, or q:");
      else
         break;
   }

   if (ch == EOF) /* make EOF cause program to quit */
      ch = 'q';

   return ch;
}

void addpet(Tree* pt)
{
   Item temp;

   if (treeIsFull(pt))
      puts("No room in the club!");
   else
   {
      puts("Please enter name of pet:");
      gets_s(temp.petname, MAX_ITEMS);
      puts("Please enter pet kind:");
      gets_s(temp.petkind, MAX_ITEMS);
      uppercase(temp.petname);
      uppercase(temp.petkind);
      addItem(&temp, pt);
   }
}

void showpets(const Tree* pt)
{
   if (treeIsEmpty(pt))
      puts("No entries!");
   else
      traverse(pt, printitem);
}

void printitem(Item item)
{
   printf("Pet: %-19s  Kind: %-19s\n",
          item.petname,
          item.petkind);
}

void findpet(const Tree* pt)
{
   Item temp;

   if (treeIsEmpty(pt))
   {
      puts("No entries!");
      return; /* quit function if tree is empty */
   }

   puts("Please enter name of pet you wish to find:");
   gets_s(temp.petname, MAX_ITEMS);
   puts("Please enter pet kind:");
   gets_s(temp.petkind,MAX_ITEMS);
   uppercase(temp.petname);
   uppercase(temp.petkind);
   printf("%s the %s ", temp.petname, temp.petkind);
   if (inTree(&temp, pt))
      printf("is a member.\n");
   else
      printf("is not a member.\n");
}

void droppet(Tree* pt)
{
   Item temp;

   if (treeIsEmpty(pt))
   {
      puts("No entries!");
      return; /* quit function if tree is empty */
   }

   puts("Please enter name of pet you wish to delete:");
   gets_s(temp.petname, MAX_ITEMS);
   puts("Please enter pet kind:");
   gets_s(temp.petkind,MAX_ITEMS);
   uppercase(temp.petname);
   uppercase(temp.petkind);
   printf("%s the %s ", temp.petname, temp.petkind);
   if (deleteItem(&temp, pt))
      printf("is dropped from the club.\n");
   else
      printf("is not a member.\n");
}

void uppercase(char* str)
{
   while (*str)
   {
      *str = toupper(*str);
      str++;
   }
}