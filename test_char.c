/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test_char.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jpepin <jpepin@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/02/17 22:23:26 by jpepin            #+#    #+#             */
/*   Updated: 2016/02/20 13:32:09 by jpepin           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdio.h>
#include <locale.h>
#include <wchar.h>

int		main(void)
{
	setlocale(LC_ALL, "");
	wchar_t *str = L"\x100"; //écrire l'unicode du caractère pour l'afficher.
	wprintf(L"%ls\n", str);
	return 0;
}
