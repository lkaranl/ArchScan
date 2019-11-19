# -*- coding: utf-8 -*-


from Tkinter import *
from ConfigParser import *
from functools import *
import os

Arq = 'Dados.txt'

FONT1 = ['Ubuntu',12,'bold']
FONT2 = ['Ubuntu',10,'bold']
FONT3 = ['Verdana',6,'bold']

class Grava_Dados(RawConfigParser):
   

    def __init__(self):
        RawConfigParser.__init__(self)

    def Verifica_Dados(self):
        if os.path.exists(Arq):
            pass
        else:
            arq = open(Arq,"w")
            arq.close()
        
    def NovoRegistro(self,Dados):
        self.Verifica_Dados()
        self.add_section("Dados")
        self.set("Dados","IP_",Dados[0]) 
        self.set("Dados","PRO",Dados[1]) 
        self.set("Dados","POR",Dados[2])
        FLconf = open(Arq,"w")
        self.write(FLconf)
        FLconf.close()

    def Pegar_Dados(self):
		self.Verifica_Dados()
		self.read(Arq)
		Open = self.get("Porta","Open")
		Close = self.get("Porta","close")
		Filte = self.get("Porta","filte")
		Opfil = self.get("Porta","opfil")
		return Open,Close,Filte,Opfil
        
def Main_Window():

	Main = Tk()
	Main['bg'] = 'white'
	Main.geometry("450x400")
	Main.resizable(False,False)
	Main.title("ArchScan")
	
	#image_fundo = PhotoImage(file = "*.png")
	#LBL_Fundo = Label(Main,image = image_fundo).place(x = 0, y = 0)
	
	LBL_Porta_Lista = Label(Main,text = 'Portas',font = FONT1,bg = 'white',fg = 'red').place(x = 10,y = 100)
	LBL_Linha_Porta = Label(Main,width = 90,bg = 'red').place(x = 0, y = 125,height = 0)
	LBL_Developers = Label(Main,text = 'Karan Luciano | Douglas',bg = 'white', fg = 'red',font = ("Verdana",6,"bold"))
	LBL_Developers.place(x = 300, y = 115)


	def Limpar():
		LIS_Filtrada.delete(0,END)
		LIS_Close.delete(0,END)
		LIS_Open.delete(0,END)
		LIS_OPFIL.delete(0,END)

	def Scan():
		IP = ETY_IP.get()
		Portas = ETY_Porta.get()
		Protocolo = ETY_Protocolo.get()
		GD = Grava_Dados()
		
		if Protocolo != 'UDP' and Protocolo != 'TCP':
			ETY_Protocolo.delete(0,END)
			ETY_Protocolo.insert(END,'TCP ou UDP')
		else:
			GD.NovoRegistro([IP,Protocolo,Portas])
			os.system("python scanner.py")
			OPEN,CLOSE,FILTER,OPFIL = GD.Pegar_Dados()
			vclose = CLOSE.split(', ')
			vopen = OPEN.split(', ')
			vfilte = FILTER.split(', ')
			vopfil = OPFIL.split(', ')

			for op in range (0,len(vopen)):
				LIS_Open.insert(END,vopen[op])

			for cl in range(0,len(vclose)):
				LIS_Close.insert(END,vclose[cl])
				
			for fi in range(0,len(vfilte)):
				LIS_Filtrada.insert(END,vfilte[fi])
			
			for of in range (0,len(vopfil)):
				LIS_OPFIL.insert(END,vopfil[of])

	


	############# DADOS
	
	LBL_IP = Label(Main,text = 'IP')
	LBL_IP['bg'] = 'white'
	LBL_IP['font'] = FONT2
	LBL_IP.place(x = 10,y = 10)
	ETY_IP = Entry(Main,width = 15)
	ETY_IP.insert(END,'127.0.0.1')
	ETY_IP['font'] = FONT2
	ETY_IP.place(x = 100,y = 10)

	LBL_Porta = Label(Main,text = 'Porta')
	LBL_Porta['bg'] = 'white'
	LBL_Porta['font'] = FONT2
	LBL_Porta.place(x = 10, y = 35)
	ETY_Porta = Entry(Main,width = 15)
	ETY_Porta.insert(END,'0-65024')
	ETY_Porta['font'] = FONT2
	ETY_Porta.place(x = 100, y = 35)

	LBL_Protocolo = Label(Main,text = 'Protocolo')
	LBL_Protocolo['bg'] = 'white'
	LBL_Protocolo['font'] = FONT2
	LBL_Protocolo.place(x= 10, y = 60)
	ETY_Protocolo = Entry(Main,width = 15)
	ETY_Protocolo.insert(END,'TCP')
	ETY_Protocolo['font'] = FONT2
	ETY_Protocolo.place(x = 100,y = 60)
	
	
	############# BOTES

	BTO_Scan = Button(Main,text = 'Scanear',width = 5,height = 3)
	BTO_Scan['command'] = Scan
	BTO_Scan['font'] = FONT1
	BTO_Scan.place(x = 272, y = 10)
	
	BTO_Limpar = Button(Main,text = 'Limpar',width = 5,height = 3)
	BTO_Limpar['font'] = FONT1
	BTO_Limpar['command'] = Limpar
	BTO_Limpar.place(x = 360, y = 10) 
	
	
	############# LISTAS
	
	LBL_Open = Label(Main,text = 'Aberta',font = FONT3,bg = 'white')
	LBL_Open.place(x = 10, y = 140)
	LIS_Open = Listbox(Main,width = 10,height = 15)
	LIS_Open.place(x = 10, y = 160)
	Scroll_Open = Scrollbar(Main,width = 15)
	Scroll_Open.configure(command=LIS_Open.yview)
	LIS_Open.configure(yscrollcommand=Scroll_Open.set)
	Scroll_Open.place(x = 95, y =160,height = 230)

	LBL_Close = Label(Main,text = 'Fechada',font = FONT3,bg = 'white')
	LBL_Close.place(x = 120, y = 140)
	LIS_Close = Listbox(Main,width = 10,height = 15)
	LIS_Close.place(x = 120, y = 160)
	Scroll_Close = Scrollbar(Main,width = 15)
	Scroll_Close.configure(command=LIS_Close.yview)
	LIS_Close.configure(yscrollcommand=Scroll_Close.set)
	Scroll_Close.place(x = 205, y =160,height = 230)

	LBL_Filtrada = Label(Main,text = 'Filtrada',font = FONT3,bg = 'white')
	LBL_Filtrada.place(x = 230, y = 140)
	LIS_Filtrada = Listbox(Main,width = 10,height = 15)
	LIS_Filtrada.place(x = 230, y = 160)
	Scroll_Filtrada = Scrollbar(Main,width = 15)
	Scroll_Filtrada.configure(command=LIS_Filtrada.yview)
	LIS_Filtrada.configure(yscrollcommand=Scroll_Filtrada.set)
	Scroll_Filtrada.place(x = 315, y =160,height = 230)

	LBL_OPFIL = Label(Main,text = 'Aberta/Filtrada',font = FONT3,bg = 'white')
	LBL_OPFIL.place(x = 340,y = 140)
	LIS_OPFIL = Listbox(Main,width = 10,height = 15)
	LIS_OPFIL.place(x = 340, y = 160)
	Scroll_OPFIL = Scrollbar(Main,width = 15)
	Scroll_OPFIL.configure(command=LIS_OPFIL.yview)
	LIS_OPFIL.configure(yscrollcommand=Scroll_OPFIL.set)
	Scroll_OPFIL.place(x = 425, y =160,height = 230)
	
	
	
	Main.mainloop()
Main_Window()
