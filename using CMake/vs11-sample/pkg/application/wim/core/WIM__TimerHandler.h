/* Generated by Together */
#ifndef WIM__TIMERHANDLER_H
#define WIM__TIMERHANDLER_H
/******************************************************************************
*=====================    Copyright by Continental AG    ======================
*******************************************************************************
* Titel        : WIM__TimerHandler.h
*
* Description  : Package private interface of timer-class:
*                Timer services
*                Private class that manages several timers needed in
*                package WIM
*
* Responsible  : L.Gruenewald, Schleissheimer GmbH
*
* Guidelines   : SMK 3.3  
*
* Template name: OOLite Codefile, Revision 1.0
*
* CASE-Tool    : Together Controlcenter, Version 6.2
*
* Revision List: (Will be filled by PVCS)
* Archive: $Log: WIM__TimerHandler.h  $
* Archive: Revision 1.1.3.2 2012/05/18 10:58:13CEST Immel-EXT, Marc (uid37561) 
* Archive: Initial revision
* Archive: Member added to project /id/dag_PF3/sw/pkg/wim/core/coreGenPF3/project.pj
* Archive: Revision 1.5 2012/05/18 10:58:13CEST uid37556 
* Archive: Member moved from WIM__TimerHandler.h in project /id/dag_PF3/sw/pkg/wim/core/project.pj[dag_PF3.sw.pkg.wim_COMMON_IC222GC_VarP] to WIM__TimerHandler.h in project /id/dag_PF3/sw/pkg/wim/core/coreGenPF3/project.pj[dag_PF3.sw.pkg.wim_COMMON_genPF3_VarX].
* Archive: Revision 1.4 2012/04/25 11:44:25CEST Gottwalles Dirk (uid37556) (uid37556) 
* Archive: DC MR 25131: W222, E007/GC, WIM: Implementation of exceptions
* Archive: DC MR 25823: BR222 E7.2: Implementation of L3 delivery (XML-Files)
* Archive: DC MR 21251: W222, E007/GC, WIM: ComplexFunction of Warning 0x2001A6 Kurvenlicht
* Archive: DC MR 25710: W222, E007/GC, WIM: Inactive handling messages show up after 600ms - WDM35020
* Archive: DC MR 25126: W222, E007/GC, WIM: Behaviour of sync acoustics - WDM35391, 35418, 35392, 35389, 35396, 35421
* Archive: Revision 1.3 2012/04/04 15:30:16CEST 07_BH_FA Partnerroom-SSH-9 (uid37556) (uid37556) 
* Archive: DC MR 24870: W222, E007/GC, WIM: Implement changes from Review ORM2907
* Archive: DC MR 25007: W222, E007/GC, WIM: Displacement of handling messages - WDM20280
* Archive: DC MR 25041: Do not specify message propagation in message description files
* Archive: DC MR 25095: W222, E007/GC, WIM: Improvement of Timer accuracy for display role - WDM 25704
* Archive: DC MR 25034: Reduction of lag times in case of vehicle locked
* Archive: DC MR 25393: W222, E007/GC, WIM: No cancel of interval acoustic - WDM31269, �LP2895
* Archive: Revision 1.2 2010/06/11 13:39:58CEST 07_BH_FA Partnerroom-SSH-9 (uid37556) (uid37556) 
* Archive: DC MR 24746: BR222 E7.1: Implementation of L3 MDB, WDB for E7.1
* Archive: DC MR 24995: IC222 GC: HMI Synchronization: Clock generation/Basic Mechanisms
* Archive: DC MR 24327: W222, E007/GC, WIM: Implement changes from Review ORM3245, ORM3314
* 
*    Rev 1.2   Jan 13 2012 16:18:56   SCHL_Dirk.Gottwalles
* DC MR 24093: W222, E007/GC, WIM: Acoustic warning does not block display - WDM20499
* DC MR 23683: W222, E007/GC, WIM: Sync acoustic is cancelled by async acoustic WDM21143
* DC MR 24088: W222, E007/GC, WIM: Warning blocks display WDM22837, WDM23219
* DC MR 24089: W222,E007,AC, WIM: New configuration values Chauffeur_Avl and ShiftStyle_Variant
* DC MR 24343: W222, E007/AC, WIM: Patch for 0x2000BC and 0x2000C1 (Country coding), WDM23191
* 
*    Rev 1.1   Jun 11 2010 14:01:08   SCHL_Thomas.Godemann
* DC MR20480 AC, Startup MR for Modul WIM in E001
* DC MR20481 GC, Startup MR for Modul WIM in E001
* 
*    Rev 1.0   Jun 07 2010 13:20:12   SCHL_Thomas.Godemann
* Initial revision.
* 
* --------------------------------------------------------------------------
* Date      | Author | Reason     | Change
* --------------------------------------------------------------------------
* 10-06-03    LarGru   MR20480      Initial revision for W222
*                      MR20481      Initial revision for W222
*
****************************************************************************/

/***************************************************************************
** Header Files (Only those that are needed in this file)
****************************************************************************/

/* System Header Files */
#include "cdef.h"

/* Foreign headerfiles */

/* Own headerfiles */
#include "WIM__TimerConfig.h"

/******************************************************************************
* Definition of exported global data used by Class
* Macros, Enumerations, Types,  Forward declarations
*******************************************************************************/

   /**
   * Correlation tag type used by WIM__Timer
   */
   typedef uint16 WIM__txCorrTag;

   /** 
   * timer callback, called if the timer has elapsed
   * @param  WIM__tenTimer: identifier of timer, WIM__txCorrTag appl. provided CorrTag
   */
   typedef void (*WIM__tpvTimerCb)(WIM__tenTimer, WIM__txCorrTag);

   /** 
   * Definition of timer configuration structure 
   */
   typedef struct
   {
      /** pointer to callback function called if timer elapses
      * @values  NULL (not used) or valid function pointer
      */
      WIM__tpvTimerCb pvCallback;  
   
      /** timer interval (loop counts) 
      * @range   1..2^32-1 
      * @values  1..2^32-1 
      */
      uint32 u32TimerInterval;


   } WIM__tstTimerConfig;

   /** 
   * Enumeration of timer states
   */
   typedef enum
   {
      WIM__nTimerInactive = 0,    /* timer was never started or was canceled */
      WIM__nTimerElapsed,         /* timer has counted out / elapsed         */
      WIM__nTimerRunning          /* timer is currently running              */
   } WIM__tenTimerState;


/******************************************************************************
* CLASS DEFINITION
*******************************************************************************/
#ifdef TOGETHER /* open C++ class */

/**
* Timer services class
*/
class WIM__TimerHandler {
public: 
#endif

/******************************************************************************
*   Class Attributes
*******************************************************************************/

/* System Wide Scope */
#ifdef TOGETHER
public:
#endif

/* File Local Scope */
#ifdef TOGETHER
private:
#endif

   /**
   * RAM table with dynamic attributes of each timer 
   */
   /*# static WIM__tstTimerState WIM__astTimerState[WIM__nMaxTimers]; */


/******************************************************************************
*   Class Operations
*******************************************************************************/

/* Package Global Scope */
#ifdef TOGETHER
public:
#endif

   /**
   * Initialisation of all timer objects (counters) 
   *   @return  void
   *   @param   none
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   void WIM__vTimerInit(void);

   /**
   * Deinitialisation of all timer objects (counters) 
   *   @return  void
   *   @param   none
   *   @author  D.Gottwalles, Schleissheimer GmbH 
   */
   void WIM__vTimerDeinit(void);

   /**
   * main routine, downcount all active timers, call registered function
   * if timer elapses
   *   @return  void
   *   @param   none
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   void WIM__vTimerMain(void);

   /**
   * service interface, start/restart a (preconfigured) timer
   *   @return  void
   *   @param   WIM__tenTimer enTimer : ID of timer to start
   *   @param   WIM__txCorrTag xCorrTag: correlation tag, return in callback function
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   void WIM__vStartTimer(WIM__tenTimer enTimer, WIM__txCorrTag xCorrTag);

   /**
   * service interface, start/restart a (preconfigured) timer with interval
   *   @return  void
   *   @param   WIM__tenTimer enTimer : ID of timer to start
   *   @param   WIM__txCorrTag xCorrTag: correlation tag, return in callback function
   *   @param   uint32        u32Int  : timer interval in ms
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   void WIM__vStartTimerEx(WIM__tenTimer enTimer, WIM__txCorrTag xCorrTag, uint32 u32Int);

   /**
   * service interface, cancel a timer; this will not cause the timer to call 
   * the configured callback function
   *   @return  void
   *   @param   WIM__tenTimer enTimer: ID of timer to stop
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   void WIM__vCancelTimer(WIM__tenTimer enTimer);

   /**
   * service interface, get status of a timer
   *   @return  WIM__tenTimerState: WIM__nRunning, WIM__nElapsed or WIM__nInactive
   *   @param   WIM__tenTimer enTimer: ID of timer to start
   *   @author  T.Godemann, Schleissheimer GmbH 
   */
   WIM__tenTimerState WIM__enGetTimer(WIM__tenTimer enTimer);

   /**
   * 
   *   @return  WIM__txCorrTag: stored tag with this timer
   *   @param   WIM__tenTimer enTimer: ID of timer to start
   *   @author  D.Gottwalles, Schleissheimer GmbH 
   */
   WIM__txCorrTag WIM__txGetTimerCorrTag(WIM__tenTimer enTimer);

/* File Local Scope */
#ifdef TOGETHER
private:
#endif

#ifdef TOGETHER /* close the C++ class */
};
#endif
/********************
**  CLASS END
*********************/
#endif /* WIM__TIMERHANDLER_H */